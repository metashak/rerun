//! The viewport panel.
//!
//! Contains all space views.
//!
//! To do:
//! * [ ] Controlling visibility of objects inside each Space View
//! * [ ] Transforming objects between spaces

use std::collections::{BTreeMap, BTreeSet};

use ahash::HashMap;
use itertools::Itertools as _;

use nohash_hasher::{IntMap, IntSet};
use re_data_store::{ObjPath, ObjPathComp, TimeInt};

use crate::{
    misc::{
        space_info::{SpaceInfo, SpacesInfo},
        Selection, ViewerContext,
    },
    ui::{view_category::group_by_category, view_spatial::SceneSpatial},
};

use super::{
    transform_cache::TransformCache, view_category::ViewCategory, SceneQuery, SpaceView,
    SpaceViewId,
};

// ----------------------------------------------------------------------------

fn query_scene_spatial(
    ctx: &mut ViewerContext<'_>,
    obj_paths: &nohash_hasher::IntSet<ObjPath>,
    transforms: &TransformCache,
) -> SceneSpatial {
    crate::profile_function!();

    let query = SceneQuery {
        obj_paths,
        timeline: *ctx.rec_cfg.time_ctrl.timeline(),
        latest_at: TimeInt::MAX,
        obj_props: &Default::default(), // all visible
    };
    let mut scene = SceneSpatial::default();
    let hovered = re_data_store::InstanceIdHash::NONE;
    scene.load_objects(ctx, &query, transforms, query.obj_props, hovered);
    scene
}

// ----------------------------------------------------------------------------

/// What views are visible?
type VisibilitySet = std::collections::BTreeSet<SpaceViewId>;

/// Describes the layout and contents of the Viewport Panel.
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Viewport {
    /// Where the space views are stored.
    space_views: HashMap<SpaceViewId, SpaceView>,

    /// Which views are visible.
    visible: VisibilitySet,

    /// The layouts of all the space views.
    ///
    /// One for each combination of what views are visible.
    /// So if a user toggles the visibility of one SpaceView, we
    /// switch which layout we are using. This is somewhat hacky.
    trees: HashMap<VisibilitySet, egui_dock::Tree<SpaceViewId>>,

    /// Show one tab as maximized?
    maximized: Option<SpaceViewId>,

    /// Set to `true` the first time the user messes around with the viewport blueprint.
    /// Before this is set we automatically add new spaces to the viewport
    /// when they show up in the data.
    has_been_user_edited: bool,
}

impl Viewport {
    /// Create a default suggested blueprint using some heuristics.
    pub fn new(ctx: &mut ViewerContext<'_>, spaces_info: &SpacesInfo) -> Self {
        crate::profile_function!();

        let mut blueprint = Self::default();

        for (path, space_info) in &spaces_info.spaces {
            // If we're connected with a rigid transform to our parent, don't create a new space view automatically,
            // since we're showing those objects in the parent by default.
            // (it is still possible to create this space view manually)
            if let Some((_, transform)) = &space_info.parent {
                match transform {
                    re_log_types::Transform::Rigid3(_) => continue,
                    re_log_types::Transform::Pinhole(_) | re_log_types::Transform::Unknown => {}
                }
            }

            let transforms = TransformCache::determine_transforms(spaces_info, space_info);

            for (category, obj_paths) in group_by_category(
                ctx.rec_cfg.time_ctrl.timeline(),
                ctx.log_db,
                space_info
                    .descendants_with_rigid_or_no_transform(spaces_info)
                    .iter(),
            ) {
                let scene_spatial = query_scene_spatial(ctx, &obj_paths, &transforms);

                if category == ViewCategory::Spatial
                    && scene_spatial.prefer_2d_mode()
                    && scene_spatial.ui.images.len() > 1
                {
                    // Multiple images (e.g. depth and rgb, or rgb and segmentation) in the same 2D scene.
                    // Stacking them on top of each other works, but is often confusing.
                    // Let's create one space view for each image, where the other images are disabled:

                    let store = &ctx.log_db.obj_db.store;

                    let mut image_sizes = BTreeSet::default();

                    for visible_image in &scene_spatial.ui.images {
                        debug_assert!(matches!(visible_image.tensor.shape.len(), 2 | 3));
                        let image_size = (
                            visible_image.tensor.shape[0].size,
                            visible_image.tensor.shape[1].size,
                        );
                        image_sizes.insert(image_size);

                        if let Some(visible_instance_id) =
                            visible_image.instance_hash.resolve(store)
                        {
                            let mut space_view = SpaceView::new(
                                ctx,
                                category,
                                path.clone(),
                                space_info,
                                spaces_info,
                                scene_spatial.preferred_navigation_mode(),
                            );
                            space_view.name = visible_instance_id.obj_path.to_string();

                            for other_image in &scene_spatial.ui.images {
                                if let Some(other_image_instance_id) =
                                    other_image.instance_hash.resolve(store)
                                {
                                    if visible_instance_id.obj_path
                                        != other_image_instance_id.obj_path
                                    {
                                        space_view
                                            .queried_objects
                                            .remove(&other_image_instance_id.obj_path);
                                        space_view.allow_auto_adding_more_object = false;
                                    }
                                }
                            }

                            blueprint.add_space_view(space_view);
                        }
                    }

                    if image_sizes.len() == 1 {
                        // All images have the same size, so we _also_ want to
                        // create the stacked version (e.g. rgb + segmentation)
                        // so we keep going here.
                    } else {
                        continue; // Different sizes, skip creating the stacked version
                    }
                }

                // Create one SpaceView for the whole space:
                {
                    let space_view = SpaceView::new(
                        ctx,
                        category,
                        path.clone(),
                        space_info,
                        spaces_info,
                        scene_spatial.preferred_navigation_mode(),
                    );
                    blueprint.add_space_view(space_view);
                }
            }
        }

        blueprint
    }

    pub(crate) fn space_view(&self, space_view: &SpaceViewId) -> Option<&SpaceView> {
        self.space_views.get(space_view)
    }

    pub(crate) fn space_view_mut(&mut self, space_view_id: &SpaceViewId) -> Option<&mut SpaceView> {
        self.space_views.get_mut(space_view_id)
    }

    pub(crate) fn remove(&mut self, space_view_id: &SpaceViewId) -> Option<SpaceView> {
        let Self {
            space_views,
            visible,
            trees,
            maximized,
            has_been_user_edited,
        } = self;

        *has_been_user_edited = true;

        trees.retain(|vis_set, _| !vis_set.contains(space_view_id));

        if *maximized == Some(*space_view_id) {
            *maximized = None;
        }

        visible.remove(space_view_id);
        space_views.remove(space_view_id)
    }

    fn has_space(&self, space_path: &ObjPath) -> bool {
        self.space_views
            .values()
            .any(|view| &view.space_path == space_path)
    }

    /// Show the blueprint panel tree view.
    pub fn tree_ui(&mut self, ctx: &mut ViewerContext<'_>, ui: &mut egui::Ui) {
        crate::profile_function!();

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                let space_view_ids = self
                    .space_views
                    .keys()
                    .sorted_by_key(|space_view_id| &self.space_views[space_view_id].name)
                    .copied()
                    .collect_vec();

                for space_view_id in &space_view_ids {
                    self.space_view_entry_ui(ctx, ui, space_view_id);
                }
            });
    }

    fn space_view_entry_ui(
        &mut self,
        ctx: &mut ViewerContext<'_>,
        ui: &mut egui::Ui,
        space_view_id: &SpaceViewId,
    ) {
        let space_view = self.space_views.get_mut(space_view_id).unwrap();
        debug_assert_eq!(space_view.id, *space_view_id);

        let collapsing_header_id = ui.id().with(space_view.id);
        egui::collapsing_header::CollapsingState::load_with_default_open(
            ui.ctx(),
            collapsing_header_id,
            true,
        )
        .show_header(ui, |ui| {
            match space_view.category {
                ViewCategory::Text => ui.label("📃"),
                ViewCategory::TimeSeries => ui.label("📈"),
                ViewCategory::BarChart => ui.label("📊"),
                ViewCategory::Spatial => match space_view.view_state.state_spatial.nav_mode {
                    super::view_spatial::SpatialNavigationMode::TwoD => ui.label("🖼"),
                    super::view_spatial::SpatialNavigationMode::ThreeD => ui.label("🔭"),
                },
                ViewCategory::Tensor => ui.label("🇹"),
            };

            if ctx
                .space_view_button_to(ui, space_view.name.clone(), *space_view_id)
                .clicked()
            {
                if let Some(tree) = self.trees.get_mut(&self.visible) {
                    focus_tab(tree, space_view_id);
                }
            }

            let mut is_space_view_visible = self.visible.contains(space_view_id);
            if visibility_button(ui, true, &mut is_space_view_visible).changed() {
                self.has_been_user_edited = true;
                if is_space_view_visible {
                    self.visible.insert(*space_view_id);
                } else {
                    self.visible.remove(space_view_id);
                }
            }
        })
        .body(|ui| {
            for path in &space_view.queried_objects {
                ui.horizontal(|ui| {
                    let name = if path.is_descendant_of(&space_view.space_path) {
                        ObjPath::from(
                            path.iter()
                                .skip(space_view.space_path.len())
                                .map(|r| r.to_owned())
                                .collect::<Vec<_>>(),
                        )
                        .to_string()
                    } else {
                        path.iter().last().unwrap().to_string()
                    };

                    ctx.space_view_obj_path_button_to(ui, name, *space_view_id, path);

                    let mut properties = space_view.obj_properties.get(path);
                    if visibility_button(ui, true, &mut properties.visible).changed() {
                        space_view.obj_properties.set(path.clone(), properties);
                    }
                });
            }
        });
    }

    pub(crate) fn mark_user_interaction(&mut self) {
        self.has_been_user_edited = true;
    }

    pub(crate) fn add_space_view(&mut self, space_view: SpaceView) -> SpaceViewId {
        let id = space_view.id;
        self.space_views.insert(id, space_view);
        self.visible.insert(id);
        self.trees.clear(); // Reset them
        id
    }

    fn add_space_view_for(
        &mut self,
        ctx: &mut ViewerContext<'_>,
        path: &ObjPath,
        space_info: &SpaceInfo,
        spaces_info: &SpacesInfo,
    ) {
        for (category, obj_paths) in group_by_category(
            ctx.rec_cfg.time_ctrl.timeline(),
            ctx.log_db,
            space_info.descendants_without_transform.iter(),
        ) {
            let scene_spatial = query_scene_spatial(
                ctx,
                &obj_paths,
                &TransformCache::determine_transforms(spaces_info, space_info),
            );
            self.add_space_view(SpaceView::new(
                ctx,
                category,
                path.clone(),
                space_info,
                spaces_info,
                scene_spatial.preferred_navigation_mode(),
            ));
        }
    }

    pub fn on_frame_start(&mut self, ctx: &mut ViewerContext<'_>, spaces_info: &SpacesInfo) {
        crate::profile_function!();

        for space_view in self.space_views.values_mut() {
            space_view.on_frame_start(ctx, spaces_info);
        }

        if !self.has_been_user_edited {
            // Automatically populate the viewport based on the data:

            if self.space_views.is_empty() {
                *self = Self::new(ctx, spaces_info);
            } else {
                crate::profile_scope!("look for missing space views");

                // Check if the blueprint is missing a space,
                // maybe one that has been added by new data:
                for (path, space_info) in &spaces_info.spaces {
                    // Ignore spaces that have a parent connected via a rigid transform to their parent,
                    // since they should be picked up automatically by existing parent spaces.
                    if let Some((_, transform)) = &space_info.parent {
                        match transform {
                            re_log_types::Transform::Rigid3(_) => continue,
                            re_log_types::Transform::Pinhole(_)
                            | re_log_types::Transform::Unknown => {}
                        }
                    }

                    if !self.has_space(path) {
                        self.add_space_view_for(ctx, path, space_info, spaces_info);
                    }
                }
            }
        }
    }

    pub fn viewport_ui(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &mut ViewerContext<'_>,
        spaces_info: &SpacesInfo,
        selection_panel_expanded: &mut bool,
    ) {
        let is_zero_sized_viewport = ui.available_size().min_elem() <= 0.0;
        if is_zero_sized_viewport {
            return;
        }

        self.trees.retain(|_, tree| is_tree_valid(tree));

        // Lazily create a layout tree based on which SpaceViews are currently visible:
        let tree = self.trees.entry(self.visible.clone()).or_insert_with(|| {
            super::auto_layout::tree_from_space_views(
                ui.available_size(),
                &self.visible,
                &self.space_views,
            )
        });

        let num_space_views = tree.num_tabs();
        if num_space_views == 0 {
            // nothing to show
        } else if num_space_views == 1 {
            let space_view_id = *tree.tabs().next().unwrap();
            let space_view = self
                .space_views
                .get_mut(&space_view_id)
                .expect("Should have been populated beforehand");

            let response = ui
                .scope(|ui| space_view_ui(ctx, ui, spaces_info, space_view))
                .response;

            let frame = ctx.re_ui.hovering_frame();
            hovering_panel(ui, frame, response.rect, |ui| {
                space_view_options_link(ctx, selection_panel_expanded, space_view.id, ui, "⛭");
            });
        } else if let Some(space_view_id) = self.maximized {
            let space_view = self
                .space_views
                .get_mut(&space_view_id)
                .expect("Should have been populated beforehand");

            let response = ui
                .scope(|ui| space_view_ui(ctx, ui, spaces_info, space_view))
                .response;

            let frame = ctx.re_ui.hovering_frame();
            hovering_panel(ui, frame, response.rect, |ui| {
                if ui
                    .button("⬅")
                    .on_hover_text("Restore - show all spaces")
                    .clicked()
                {
                    self.maximized = None;
                }
                space_view_options_link(ctx, selection_panel_expanded, space_view.id, ui, "⛭");
            });
        } else {
            let mut dock_style = egui_dock::Style::from_egui(ui.style().as_ref());
            dock_style.separator_width = 2.0;
            dock_style.default_inner_margin = 0.0.into();
            dock_style.show_close_buttons = false;
            dock_style.tab_include_scrollarea = false;
            // dock_style.expand_tabs = true; looks good, but decreases readability
            dock_style.tab_text_color_unfocused = dock_style.tab_text_color_focused; // We don't treat focused tabs differently
            dock_style.tab_background_color = ui.visuals().panel_fill;

            let mut tab_viewer = TabViewer {
                ctx,
                spaces_info,
                space_views: &mut self.space_views,
                maximized: &mut self.maximized,
                selection_panel_expanded,
            };

            egui_dock::DockArea::new(tree)
                .style(dock_style)
                .show_inside(ui, &mut tab_viewer);
        }
    }

    pub fn create_new_blueprint_ui(
        &mut self,
        ctx: &mut ViewerContext<'_>,
        ui: &mut egui::Ui,
        spaces_info: &SpacesInfo,
    ) {
        #![allow(clippy::collapsible_if)]

        ui.vertical_centered(|ui| {
            ui.menu_button("Add new space view…", |ui| {
                ui.style_mut().wrap = Some(false);

                let objects_per_root_grouped_by_category =
                    objects_per_root_grouped_by_category(ctx);

                for (path, space_info) in &spaces_info.spaces {
                    let categories = objects_per_root_grouped_by_category
                        .get(&ObjPath::from(&path.to_components()[..1]));

                    if let Some(categories) = categories {
                        if categories.is_empty() {
                            continue;
                        }

                        let transforms =
                            TransformCache::determine_transforms(spaces_info, space_info);

                        if ui.button(path.to_string()).clicked() {
                            ui.close_menu();

                            for (category, obj_paths) in categories {
                                let scene_spatial =
                                    query_scene_spatial(ctx, obj_paths, &transforms);
                                let new_space_view_id = self.add_space_view(SpaceView::new(
                                    ctx,
                                    *category,
                                    path.clone(),
                                    space_info,
                                    spaces_info,
                                    scene_spatial.preferred_navigation_mode(),
                                ));
                                ctx.set_selection(Selection::SpaceView(new_space_view_id));
                            }
                        }
                    }
                }
            });
        });
    }
}

/// Returns iterator over all root paths, each with a list of object paths under it.
fn objects_per_root<'a>(
    ctx: &mut ViewerContext<'a>,
) -> impl Iterator<Item = (ObjPath, Vec<ObjPath>)> + 'a {
    let roots = &ctx.log_db.obj_db.tree.children;
    roots.iter().map(|(root_path, root_tree)| {
        let mut objects = Vec::new();
        root_tree.visit_children_recursively(&mut |child_path| {
            objects.push(child_path.clone());
        });
        let root_path: &[ObjPathComp] = &[root_path.clone()];
        (ObjPath::from(root_path), objects)
    })
}

/// Returns all map from all root paths to objects grouped by category.
fn objects_per_root_grouped_by_category(
    ctx: &mut ViewerContext<'_>,
) -> IntMap<ObjPath, BTreeMap<ViewCategory, IntSet<ObjPath>>> {
    objects_per_root(ctx)
        .map(|(root, objects)| {
            (
                root,
                group_by_category(ctx.rec_cfg.time_ctrl.timeline(), ctx.log_db, objects.iter()),
            )
        })
        .collect()
}

fn visibility_button(ui: &mut egui::Ui, enabled: bool, visible: &mut bool) -> egui::Response {
    ui.add_space(16.0); // Make room for visibility button so the side bar don't become too narrow to fit it

    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.set_enabled(enabled);
        if enabled {
            ui.toggle_value(visible, "👁")
        } else {
            let mut always_false = false;
            ui.toggle_value(&mut always_false, "👁")
        }
        .on_hover_text("Toggle visibility")
    })
    .inner
}

// ----------------------------------------------------------------------------

struct TabViewer<'a, 'b> {
    ctx: &'a mut ViewerContext<'b>,
    spaces_info: &'a SpacesInfo,
    space_views: &'a mut HashMap<SpaceViewId, SpaceView>,
    maximized: &'a mut Option<SpaceViewId>,
    selection_panel_expanded: &'a mut bool,
}

impl<'a, 'b> egui_dock::TabViewer for TabViewer<'a, 'b> {
    type Tab = SpaceViewId;

    fn ui(&mut self, ui: &mut egui::Ui, space_view_id: &mut Self::Tab) {
        crate::profile_function!();

        let space_view = self
            .space_views
            .get_mut(space_view_id)
            .expect("Should have been populated beforehand");

        let response = ui
            .scope(|ui| space_view_ui(self.ctx, ui, self.spaces_info, space_view))
            .response;

        // Show buttons for maximize and space view options:
        let frame = self.ctx.re_ui.hovering_frame();
        hovering_panel(ui, frame, response.rect, |ui| {
            if ui
                .button("🗖")
                .on_hover_text("Maximize Space View")
                .clicked()
            {
                *self.maximized = Some(*space_view_id);
                self.ctx.set_selection(Selection::SpaceView(*space_view_id));
            }

            space_view_options_link(
                self.ctx,
                self.selection_panel_expanded,
                *space_view_id,
                ui,
                "⛭",
            );
        });
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        let space_view = self
            .space_views
            .get_mut(tab)
            .expect("Should have been populated beforehand");
        space_view.name.clone().into()
    }
}

fn space_view_options_link(
    ctx: &mut ViewerContext<'_>,
    selection_panel_expanded: &mut bool,
    space_view_id: SpaceViewId,
    ui: &mut egui::Ui,
    text: &str,
) {
    let is_selected =
        ctx.selection() == Selection::SpaceView(space_view_id) && *selection_panel_expanded;
    if ui
        .selectable_label(is_selected, text)
        .on_hover_text("Space View options")
        .clicked()
    {
        if is_selected {
            ctx.clear_selection();
            *selection_panel_expanded = false;
        } else {
            ctx.set_selection(Selection::SpaceView(space_view_id));
            *selection_panel_expanded = true;
        }
    }
}

fn hovering_panel(
    ui: &mut egui::Ui,
    frame: egui::Frame,
    rect: egui::Rect,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    let mut ui = ui.child_ui(rect, egui::Layout::top_down(egui::Align::LEFT));
    ui.horizontal(|ui| {
        frame.show(ui, add_contents);
    });
}

fn space_view_ui(
    ctx: &mut ViewerContext<'_>,
    ui: &mut egui::Ui,
    spaces_info: &SpacesInfo,
    space_view: &mut SpaceView,
) {
    let Some(reference_space_info) = spaces_info.spaces.get(&space_view.space_path) else {
        ui.centered_and_justified(|ui| {
            ui.label(ctx.re_ui.warning_text(format!("Unknown space {}", space_view.space_path)));
        });
        return;
    };

    let Some(latest_at) = ctx.rec_cfg.time_ctrl.time_int() else {
        ui.centered_and_justified(|ui| {
            ui.label(ctx.re_ui.warning_text("No time selected"));
        });
        return
    };

    space_view.scene_ui(ctx, ui, spaces_info, reference_space_info, latest_at);
}

// ----------------------------------------------------------------------------

fn focus_tab(tree: &mut egui_dock::Tree<SpaceViewId>, tab: &SpaceViewId) {
    if let Some((node_index, tab_index)) = tree.find_tab(tab) {
        tree.set_focused_node(node_index);
        tree.set_active_tab(node_index, tab_index);
    }
}

fn is_tree_valid(tree: &egui_dock::Tree<SpaceViewId>) -> bool {
    tree.iter().all(|node| match node {
        egui_dock::Node::Vertical { rect: _, fraction }
        | egui_dock::Node::Horizontal { rect: _, fraction } => fraction.is_finite(),
        _ => true,
    })
}
