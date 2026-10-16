#include <QtCore/QObject>
#include "cxx-qt/connection.h"
#include "private/src/qml/qqmlimageproviderbase.cxxqt.h"
#include "cxx-qt-lib/qqmlimageproviderbase.h"
#include <cstdint>
#include <type_traits>

#ifdef __GNUC__
#pragma GCC diagnostic ignored "-Wmissing-declarations"
#ifdef __clang__
#pragma clang diagnostic ignored "-Wdollar-in-identifier-extension"
#endif // __clang__
#endif // __GNUC__

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

namespace cxxqt1 {
extern "C" {
::QObject const *rust$cxxqt1$cxxbridge1$205$cxx_qt_ffi_QQmlImageProviderBase_upcastPtr(::QQmlImageProviderBase const *thiz) noexcept {
  ::QObject const *(*cxx_qt_ffi_QQmlImageProviderBase_upcastPtr$)(::QQmlImageProviderBase const *) = ::rust::cxxqt1::upcastPtr;
  return cxx_qt_ffi_QQmlImageProviderBase_upcastPtr$(thiz);
}

::QQmlImageProviderBase const *rust$cxxqt1$cxxbridge1$205$cxx_qt_ffi_QQmlImageProviderBase_downcastPtr(::QObject const *base) noexcept {
  ::QQmlImageProviderBase const *(*cxx_qt_ffi_QQmlImageProviderBase_downcastPtr$)(::QObject const *) = ::rust::cxxqt1::downcastPtr;
  return cxx_qt_ffi_QQmlImageProviderBase_downcastPtr$(base);
}
} // extern "C"
} // namespace cxxqt1
} // namespace rust
