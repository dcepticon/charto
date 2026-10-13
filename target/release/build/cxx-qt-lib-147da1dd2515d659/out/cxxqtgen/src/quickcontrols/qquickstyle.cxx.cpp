#include "cxx-qt-lib/qquickstyle.h"
#include "cxx-qt-lib/qstring.h"
#include <QtCore/QObject>
#include "cxx-qt/connection.h"
#include "private/src/quickcontrols/qquickstyle.cxxqt.h"
#include <new>
#include <type_traits>

#ifdef __GNUC__
#pragma GCC diagnostic ignored "-Wmissing-declarations"
#ifdef __clang__
#pragma clang diagnostic ignored "-Wdollar-in-identifier-extension"
#endif // __clang__
#endif // __GNUC__

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

#ifndef CXXBRIDGE1_RELOCATABLE
#define CXXBRIDGE1_RELOCATABLE
namespace detail {
template <typename... Ts>
struct make_void {
  using type = void;
};

template <typename... Ts>
using void_t = typename make_void<Ts...>::type;

template <typename Void, template <typename...> class, typename...>
struct detect : std::false_type {};
template <template <typename...> class T, typename... A>
struct detect<void_t<T<A...>>, T, A...> : std::true_type {};

template <template <typename...> class T, typename... A>
using is_detected = detect<void, T, A...>;

template <typename T>
using detect_IsRelocatable = typename T::IsRelocatable;

template <typename T>
struct get_IsRelocatable
    : std::is_same<typename T::IsRelocatable, std::true_type> {};
} // namespace detail

template <typename T>
struct IsRelocatable
    : std::conditional<
          detail::is_detected<detail::detect_IsRelocatable, T>::value,
          detail::get_IsRelocatable<T>,
          std::integral_constant<
              bool, std::is_trivially_move_constructible<T>::value &&
                        std::is_trivially_destructible<T>::value>>::type {};
#endif // CXXBRIDGE1_RELOCATABLE
} // namespace cxxbridge1
} // namespace rust

using QQuickStyle = ::QQuickStyle;

static_assert(
    ::rust::IsRelocatable<::QString>::value,
    "type QString should be trivially move constructible and trivially destructible in C++ to be used as a return value of `name` in Rust");

extern "C" {
void cxxbridge1$205$QQuickStyle$name(::QString *return$) noexcept {
  ::QString (*name$)() = &::QQuickStyle::name;
  new (return$) ::QString(name$());
}

void cxxbridge1$205$QQuickStyle$set_fallback_style(::QString const &style) noexcept {
  void (*set_fallback_style$)(::QString const &) = &::QQuickStyle::setFallbackStyle;
  set_fallback_style$(style);
}

void cxxbridge1$205$QQuickStyle$set_style(::QString const &style) noexcept {
  void (*set_style$)(::QString const &) = &::QQuickStyle::setStyle;
  set_style$(style);
}
} // extern "C"
