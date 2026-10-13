#pragma once
#include "cxx-qt-lib/qt.h"
#include "cxx-qt-lib/qregion.h"
#include "cxx-qt-lib/qrect.h"
#include "cxx-qt-lib/qpoint.h"
#include "cxx-qt-lib/qpolygon.h"
#include "cxx-qt-lib/common.h"
#include <cstdint>
#include <type_traits>

namespace rust {
  namespace cxxqtlib1 {
    using QRegionRegionType = ::rust::cxxqtlib1::QRegionRegionType;
  }
}

namespace rust {
namespace cxxqtlib1 {
static_assert(::std::is_enum<QRegionRegionType>::value, "expected enum");
static_assert(sizeof(QRegionRegionType) == sizeof(::std::int32_t), "incorrect size");
static_assert(static_cast<::std::int32_t>(QRegionRegionType::Rectangle) == 0, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QRegionRegionType::Ellipse) == 1, "disagrees with the value in #[cxx::bridge]");
} // namespace cxxqtlib1
} // namespace rust
