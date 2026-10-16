#pragma once
#include <QtCore/QObject>
#include "cxx-qt/connection.h"
#include "private/src/qml/qqmlimageproviderbase.cxxqt.h"
#include "cxx-qt-lib/qqmlimageproviderbase.h"
#include <cstdint>
#include <type_traits>

using QQmlImageProviderBase = ::QQmlImageProviderBase;
namespace rust {
  namespace cxxqtlib1 {
    using QQmlImageProviderBaseImageType = ::rust::cxxqtlib1::QQmlImageProviderBaseImageType;
  }
}

namespace rust {
namespace cxxqtlib1 {
static_assert(::std::is_enum<QQmlImageProviderBaseImageType>::value, "expected enum");
static_assert(sizeof(QQmlImageProviderBaseImageType) == sizeof(::std::int32_t), "incorrect size");
static_assert(static_cast<::std::int32_t>(QQmlImageProviderBaseImageType::Invalid) == 0, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QQmlImageProviderBaseImageType::Image) == 1, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QQmlImageProviderBaseImageType::Pixmap) == 2, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QQmlImageProviderBaseImageType::Texture) == 3, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QQmlImageProviderBaseImageType::ImageResponse) == 4, "disagrees with the value in #[cxx::bridge]");
} // namespace cxxqtlib1
} // namespace rust
