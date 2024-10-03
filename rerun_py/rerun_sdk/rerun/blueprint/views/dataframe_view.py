# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/views/dataframe.fbs".

from __future__ import annotations

from typing import Union

__all__ = ["DataframeView"]


from ... import datatypes
from ..._baseclasses import AsComponents, ComponentBatchLike
from ...datatypes import EntityPathLike, Utf8Like
from .. import archetypes as blueprint_archetypes
from ..api import SpaceView, SpaceViewContentsLike


class DataframeView(SpaceView):
    """
    **View**: A view to display any data in a tabular form.

    Any data from the store can be shown, using a flexibly, user-configurable query.

    Example
    -------
    ### Use a blueprint to customize a DataframeView.:
    ```python
    import math

    import rerun as rr
    import rerun.blueprint as rrb

    rr.init("rerun_example_dataframe", spawn=True)

    # Log some data.
    for t in range(0, int(math.pi * 4 * 100.0)):
        rr.set_time_seconds("t", t)
        rr.log("trig/sin", rr.Scalar(math.sin(float(t) / 100.0)))
        rr.log("trig/cos", rr.Scalar(math.cos(float(t) / 100.0)))

        # some sparse data
        if t % 5 == 0:
            rr.log("trig/tan_sparse", rr.Scalar(math.tan(float(t) / 100.0)))

    # Create a Dataframe View
    blueprint = rrb.Blueprint(
        rrb.DataframeView(
            origin="/trig",
            query=rrb.archetypes.DataframeQueryV2(
                timeline="t",
                filter_by_range=(rr.TimeInt(seconds=0), rr.TimeInt(seconds=20)),
                filter_by_event="/trig/tan_sparse:Scalar",
                select=["t", "log_tick", "/trig/sin:Scalar", "/trig/cos:Scalar", "/trig/tan_sparse:Scalar"],
            ),
        ),
    )

    rr.send_blueprint(blueprint)
    ```

    """

    def __init__(
        self,
        *,
        origin: EntityPathLike = "/",
        contents: SpaceViewContentsLike = "$origin/**",
        name: Utf8Like | None = None,
        visible: datatypes.BoolLike | None = None,
        defaults: list[Union[AsComponents, ComponentBatchLike]] = [],
        overrides: dict[EntityPathLike, list[ComponentBatchLike]] = {},
        query: blueprint_archetypes.DataframeQueryV2 | None = None,
    ) -> None:
        """
        Construct a blueprint for a new DataframeView view.

        Parameters
        ----------
        origin:
            The `EntityPath` to use as the origin of this view.
            All other entities will be transformed to be displayed relative to this origin.
        contents:
            The contents of the view specified as a query expression.
            This is either a single expression, or a list of multiple expressions.
            See [rerun.blueprint.archetypes.SpaceViewContents][].
        name:
            The display name of the view.
        visible:
            Whether this view is visible.

            Defaults to true if not specified.
        defaults:
            List of default components or component batches to add to the space view. When an archetype
            in the view is missing a component included in this set, the value of default will be used
            instead of the normal fallback for the visualizer.
        overrides:
            Dictionary of overrides to apply to the space view. The key is the path to the entity where the override
            should be applied. The value is a list of component or component batches to apply to the entity.

            Important note: the path must be a fully qualified entity path starting at the root. The override paths
            do not yet support `$origin` relative paths or glob expressions.
            This will be addressed in <https://github.com/rerun-io/rerun/issues/6673>.
        query:
            Query of the dataframe.

        """

        properties: dict[str, AsComponents] = {}
        if query is not None:
            if not isinstance(query, blueprint_archetypes.DataframeQueryV2):
                query = blueprint_archetypes.DataframeQueryV2(query)
            properties["DataframeQueryV2"] = query

        super().__init__(
            class_identifier="Dataframe",
            origin=origin,
            contents=contents,
            name=name,
            visible=visible,
            properties=properties,
            defaults=defaults,
            overrides=overrides,
        )
