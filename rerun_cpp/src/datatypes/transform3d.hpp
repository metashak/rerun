// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/datatypes/transform3d.fbs"

#pragma once

#include <cstdint>
#include <cstring>
#include <utility>

#include "../datatypes/translation_and_mat3x3.hpp"
#include "../datatypes/translation_rotation_scale3d.hpp"

namespace rr {
    namespace datatypes {
        namespace detail {
            enum class Transform3DTag {
                /// Having a special empty state makes it possible to implement move-semantics. We
                /// need to be able to leave the object in a state which we can run the destructor
                /// on.
                NONE = 0,
                TranslationAndMat3x3,
                TranslationRotationScale,
            };

            union Transform3DData {
                rr::datatypes::TranslationAndMat3x3 translation_and_mat3x3;

                rr::datatypes::TranslationRotationScale3D translation_rotation_scale;

                Transform3DData() {}

                ~Transform3DData() {}

                void swap(Transform3DData& other) noexcept {
                    // This bitwise swap would fail for self-referential types, but we don't have
                    // any of those.
                    char temp[sizeof(Transform3DData)];
                    std::memcpy(temp, this, sizeof(Transform3DData));
                    std::memcpy(this, &other, sizeof(Transform3DData));
                    std::memcpy(&other, temp, sizeof(Transform3DData));
                }
            };
        } // namespace detail

        /// Representation of a 3D affine transform.
        struct Transform3D {
          private:
            detail::Transform3DTag _tag;
            detail::Transform3DData _data;

            Transform3D() : _tag(detail::Transform3DTag::NONE) {}

          public:
            Transform3D(Transform3D&& other) noexcept : _tag(detail::Transform3DTag::NONE) {
                this->swap(other);
            }

            Transform3D& operator=(Transform3D&& other) noexcept {
                this->swap(other);
                return *this;
            }

            static Transform3D translation_and_mat3x3(
                rr::datatypes::TranslationAndMat3x3 translation_and_mat3x3) {
                Transform3D self;
                self._tag = detail::Transform3DTag::TranslationAndMat3x3;
                self._data.translation_and_mat3x3 = std::move(translation_and_mat3x3);
                return std::move(self);
            }

            static Transform3D translation_rotation_scale(
                rr::datatypes::TranslationRotationScale3D translation_rotation_scale) {
                Transform3D self;
                self._tag = detail::Transform3DTag::TranslationRotationScale;
                self._data.translation_rotation_scale = std::move(translation_rotation_scale);
                return std::move(self);
            }

            Transform3D(rr::datatypes::TranslationAndMat3x3 translation_and_mat3x3) {
                *this = Transform3D::translation_and_mat3x3(std::move(translation_and_mat3x3));
            }

            Transform3D(rr::datatypes::TranslationRotationScale3D translation_rotation_scale) {
                *this =
                    Transform3D::translation_rotation_scale(std::move(translation_rotation_scale));
            }

            void swap(Transform3D& other) noexcept {
                auto tag_temp = this->_tag;
                this->_tag = other._tag;
                other._tag = tag_temp;
                this->_data.swap(other._data);
            }
        };
    } // namespace datatypes
} // namespace rr
