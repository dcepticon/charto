#include "cxx-qt-lib/qjsonarray.h"
#include "cxx-qt-lib/qjsonvalue.h"
#include "cxx-qt-lib/qstring.h"
#include "cxx-qt-lib/common.h"
#include <new>
#include <type_traits>
#if defined(_WIN32)
#include <basetsd.h>
#else
#include <sys/types.h>
#endif

#ifdef __GNUC__
#pragma GCC diagnostic ignored "-Wmissing-declarations"
#ifdef __clang__
#pragma clang diagnostic ignored "-Wdollar-in-identifier-extension"
#endif // __clang__
#endif // __GNUC__

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

#ifndef CXXBRIDGE1_RUST_ISIZE
#define CXXBRIDGE1_RUST_ISIZE
#if defined(_WIN32)
using isize = SSIZE_T;
#else
using isize = ssize_t;
#endif
#endif // CXXBRIDGE1_RUST_ISIZE

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
    ::rust::IsRelocatable<::QJsonArray>::value,
    "type QJsonArray should be trivially move constructible and trivially destructible in C++ to be used as a return value of `qjsonarray_init_default`, `qjsonarray_init_from_qjsonarray` in Rust");
static_assert(
    ::rust::IsRelocatable<::QJsonValue>::value,
    "type QJsonValue should be trivially move constructible and trivially destructible in C++ to be used as a return value of `qjsonarray_at` in Rust");
static_assert(
    ::rust::IsRelocatable<::QString>::value,
    "type QString should be trivially move constructible and trivially destructible in C++ to be used as a return value of `qjsonarray_to_debug_qstring` in Rust");

extern "C" {
void cxxbridge1$205$QJsonArray$append(::QJsonArray &self, ::QJsonValue const &value) noexcept {
  void (::QJsonArray::*append$)(::QJsonValue const &) = &::QJsonArray::append;
  (self.*append$)(value);
}
} // extern "C"

namespace rust {
namespace cxxqtlib1 {
extern "C" {
void rust$cxxqtlib1$cxxbridge1$205$qjsonarray_drop(::QJsonArray &qjsonarray) noexcept {
  void (*qjsonarray_drop$)(::QJsonArray &) = ::rust::cxxqtlib1::drop;
  qjsonarray_drop$(qjsonarray);
}

void rust$cxxqtlib1$cxxbridge1$205$qjsonarray_init_default(::QJsonArray *return$) noexcept {
  ::QJsonArray (*qjsonarray_init_default$)() = ::rust::cxxqtlib1::construct;
  new (return$) ::QJsonArray(qjsonarray_init_default$());
}

void rust$cxxqtlib1$cxxbridge1$205$qjsonarray_init_from_qjsonarray(::QJsonArray const &qjsonarray, ::QJsonArray *return$) noexcept {
  ::QJsonArray (*qjsonarray_init_from_qjsonarray$)(::QJsonArray const &) = ::rust::cxxqtlib1::construct;
  new (return$) ::QJsonArray(qjsonarray_init_from_qjsonarray$(qjsonarray));
}

bool rust$cxxqtlib1$cxxbridge1$205$qjsonarray_eq(::QJsonArray const &a, ::QJsonArray const &b) noexcept {
  bool (*qjsonarray_eq$)(::QJsonArray const &, ::QJsonArray const &) = ::rust::cxxqtlib1::operatorEq;
  return qjsonarray_eq$(a, b);
}

void rust$cxxqtlib1$cxxbridge1$205$qjsonarray_to_debug_qstring(::QJsonArray const &array, ::QString *return$) noexcept {
  ::QString (*qjsonarray_to_debug_qstring$)(::QJsonArray const &) = ::rust::cxxqtlib1::toDebugQString;
  new (return$) ::QString(qjsonarray_to_debug_qstring$(array));
}

::rust::isize rust$cxxqtlib1$cxxbridge1$205$qjsonarray_len(::QJsonArray const &array) noexcept {
  ::rust::isize (*qjsonarray_len$)(::QJsonArray const &) = ::rust::cxxqtlib1::qjsonarrayLen;
  return qjsonarray_len$(array);
}

void rust$cxxqtlib1$cxxbridge1$205$qjsonarray_at(::QJsonArray const &array, ::rust::isize i, ::QJsonValue *return$) noexcept {
  ::QJsonValue (*qjsonarray_at$)(::QJsonArray const &, ::rust::isize) = ::rust::cxxqtlib1::qjsonarrayAt;
  new (return$) ::QJsonValue(qjsonarray_at$(array, i));
}
} // extern "C"
} // namespace cxxqtlib1
} // namespace rust
