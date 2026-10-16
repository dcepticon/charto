#include "cxx-qt-lib/core/qhash/qhash_QString_QVariant.h"
#include "cxx-qt-lib/qvariant.h"
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

static_assert(
    ::rust::IsRelocatable<::QHash_QString_QVariant>::value,
    "type QHash_QString_QVariant should be trivially move constructible and trivially destructible in C++ to be used as a return value of `value_or_default_QVariantHash` in Rust");
static_assert(
    ::rust::IsRelocatable<::QVariant>::value,
    "type QVariant should be trivially move constructible and trivially destructible in C++ to be used as a return value of `construct_QVariantHash` in Rust");

namespace rust {
namespace cxxqtlib1 {
namespace qvariant {
extern "C" {
bool rust$cxxqtlib1$qvariant$cxxbridge1$205$can_convert_QVariantHash(::QVariant const &variant) noexcept {
  bool (*can_convert_QVariantHash$)(::QVariant const &) = ::rust::cxxqtlib1::qvariant::qvariantCanConvertQVariantHash;
  return can_convert_QVariantHash$(variant);
}

void rust$cxxqtlib1$qvariant$cxxbridge1$205$construct_QVariantHash(::QHash_QString_QVariant const &value, ::QVariant *return$) noexcept {
  ::QVariant (*construct_QVariantHash$)(::QHash_QString_QVariant const &) = ::rust::cxxqtlib1::qvariant::qvariantConstruct;
  new (return$) ::QVariant(construct_QVariantHash$(value));
}

void rust$cxxqtlib1$qvariant$cxxbridge1$205$value_or_default_QVariantHash(::QVariant const &variant, ::QHash_QString_QVariant *return$) noexcept {
  ::QHash_QString_QVariant (*value_or_default_QVariantHash$)(::QVariant const &) = ::rust::cxxqtlib1::qvariant::qvariantValueOrDefault;
  new (return$) ::QHash_QString_QVariant(value_or_default_QVariantHash$(variant));
}
} // extern "C"
} // namespace qvariant
} // namespace cxxqtlib1
} // namespace rust
