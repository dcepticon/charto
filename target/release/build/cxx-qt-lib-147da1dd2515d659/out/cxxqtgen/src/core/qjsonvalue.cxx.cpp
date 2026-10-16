#include "cxx-qt-lib/qjsonvalue.h"
#include "cxx-qt-lib/qjsonarray.h"
#include "cxx-qt-lib/qjsonobject.h"
#include "cxx-qt-lib/qstring.h"
#include "cxx-qt-lib/common.h"
#include <cstdint>
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
    ::rust::IsRelocatable<::QJsonValue>::value,
    "type QJsonValue should be trivially move constructible and trivially destructible in C++ to be used as a return value of `qjsonvalue_init_default`, `qjsonvalue_init_from_qjsonvalue`, `qjsonvalue_init_from_bool` in Rust");
static_assert(
    ::rust::IsRelocatable<::QJsonArray>::value,
    "type QJsonArray should be trivially move constructible and trivially destructible in C++ to be used as a return value of `to_array_or` in Rust");
static_assert(
    ::rust::IsRelocatable<::QJsonObject>::value,
    "type QJsonObject should be trivially move constructible and trivially destructible in C++ to be used as a return value of `to_object_or` in Rust");
static_assert(
    ::rust::IsRelocatable<::QString>::value,
    "type QString should be trivially move constructible and trivially destructible in C++ to be used as a return value of `to_string_or`, `qjsonvalue_to_debug_qstring` in Rust");

extern "C" {
bool cxxbridge1$205$QJsonValue$is_null(::QJsonValue const &self) noexcept {
  bool (::QJsonValue::*is_null$)() const = &::QJsonValue::isNull;
  return (self.*is_null$)();
}

bool cxxbridge1$205$QJsonValue$is_undefined(::QJsonValue const &self) noexcept {
  bool (::QJsonValue::*is_undefined$)() const = &::QJsonValue::isUndefined;
  return (self.*is_undefined$)();
}

bool cxxbridge1$205$QJsonValue$is_bool(::QJsonValue const &self) noexcept {
  bool (::QJsonValue::*is_bool$)() const = &::QJsonValue::isBool;
  return (self.*is_bool$)();
}

bool cxxbridge1$205$QJsonValue$is_double(::QJsonValue const &self) noexcept {
  bool (::QJsonValue::*is_double$)() const = &::QJsonValue::isDouble;
  return (self.*is_double$)();
}

bool cxxbridge1$205$QJsonValue$is_string(::QJsonValue const &self) noexcept {
  bool (::QJsonValue::*is_string$)() const = &::QJsonValue::isString;
  return (self.*is_string$)();
}

bool cxxbridge1$205$QJsonValue$is_array(::QJsonValue const &self) noexcept {
  bool (::QJsonValue::*is_array$)() const = &::QJsonValue::isArray;
  return (self.*is_array$)();
}

bool cxxbridge1$205$QJsonValue$is_object(::QJsonValue const &self) noexcept {
  bool (::QJsonValue::*is_object$)() const = &::QJsonValue::isObject;
  return (self.*is_object$)();
}

bool cxxbridge1$205$QJsonValue$to_bool_or(::QJsonValue const &self, bool default_value) noexcept {
  bool (::QJsonValue::*to_bool_or$)(bool) const = &::QJsonValue::toBool;
  return (self.*to_bool_or$)(default_value);
}

double cxxbridge1$205$QJsonValue$to_double_or(::QJsonValue const &self, double default_value) noexcept {
  double (::QJsonValue::*to_double_or$)(double) const = &::QJsonValue::toDouble;
  return (self.*to_double_or$)(default_value);
}

::std::int32_t cxxbridge1$205$QJsonValue$to_int_or(::QJsonValue const &self, ::std::int32_t default_value) noexcept {
  ::std::int32_t (::QJsonValue::*to_int_or$)(::std::int32_t) const = &::QJsonValue::toInt;
  return (self.*to_int_or$)(default_value);
}

void cxxbridge1$205$QJsonValue$to_string_or(::QJsonValue const &self, ::QString const &default_value, ::QString *return$) noexcept {
  ::QString (::QJsonValue::*to_string_or$)(::QString const &) const = &::QJsonValue::toString;
  new (return$) ::QString((self.*to_string_or$)(default_value));
}

void cxxbridge1$205$QJsonValue$to_array_or(::QJsonValue const &self, ::QJsonArray const &default_value, ::QJsonArray *return$) noexcept {
  ::QJsonArray (::QJsonValue::*to_array_or$)(::QJsonArray const &) const = &::QJsonValue::toArray;
  new (return$) ::QJsonArray((self.*to_array_or$)(default_value));
}

void cxxbridge1$205$QJsonValue$to_object_or(::QJsonValue const &self, ::QJsonObject const &default_value, ::QJsonObject *return$) noexcept {
  ::QJsonObject (::QJsonValue::*to_object_or$)(::QJsonObject const &) const = &::QJsonValue::toObject;
  new (return$) ::QJsonObject((self.*to_object_or$)(default_value));
}
} // extern "C"

namespace rust {
namespace cxxqtlib1 {
extern "C" {
void rust$cxxqtlib1$cxxbridge1$205$qjsonvalue_drop(::QJsonValue &value) noexcept {
  void (*qjsonvalue_drop$)(::QJsonValue &) = ::rust::cxxqtlib1::drop;
  qjsonvalue_drop$(value);
}

void rust$cxxqtlib1$cxxbridge1$205$qjsonvalue_init_default(::QJsonValue *return$) noexcept {
  ::QJsonValue (*qjsonvalue_init_default$)() = ::rust::cxxqtlib1::construct;
  new (return$) ::QJsonValue(qjsonvalue_init_default$());
}

void rust$cxxqtlib1$cxxbridge1$205$qjsonvalue_init_from_qjsonvalue(::QJsonValue const &value, ::QJsonValue *return$) noexcept {
  ::QJsonValue (*qjsonvalue_init_from_qjsonvalue$)(::QJsonValue const &) = ::rust::cxxqtlib1::construct;
  new (return$) ::QJsonValue(qjsonvalue_init_from_qjsonvalue$(value));
}

void rust$cxxqtlib1$cxxbridge1$205$qjsonvalue_init_from_bool(bool value, ::QJsonValue *return$) noexcept {
  ::QJsonValue (*qjsonvalue_init_from_bool$)(bool) = ::rust::cxxqtlib1::construct;
  new (return$) ::QJsonValue(qjsonvalue_init_from_bool$(value));
}

void rust$cxxqtlib1$cxxbridge1$205$qjsonvalue_init_from_i64(::std::int64_t value, ::QJsonValue *return$) noexcept {
  ::QJsonValue (*qjsonvalue_init_from_i64$)(::std::int64_t) = ::rust::cxxqtlib1::qjsonvalueFromI64;
  new (return$) ::QJsonValue(qjsonvalue_init_from_i64$(value));
}

void rust$cxxqtlib1$cxxbridge1$205$qjsonvalue_init_from_f64(double value, ::QJsonValue *return$) noexcept {
  ::QJsonValue (*qjsonvalue_init_from_f64$)(double) = ::rust::cxxqtlib1::construct;
  new (return$) ::QJsonValue(qjsonvalue_init_from_f64$(value));
}

void rust$cxxqtlib1$cxxbridge1$205$qjsonvalue_init_from_qstring(::QString const &value, ::QJsonValue *return$) noexcept {
  ::QJsonValue (*qjsonvalue_init_from_qstring$)(::QString const &) = ::rust::cxxqtlib1::construct;
  new (return$) ::QJsonValue(qjsonvalue_init_from_qstring$(value));
}

void rust$cxxqtlib1$cxxbridge1$205$qjsonvalue_init_from_qjsonarray(::QJsonArray const &value, ::QJsonValue *return$) noexcept {
  ::QJsonValue (*qjsonvalue_init_from_qjsonarray$)(::QJsonArray const &) = ::rust::cxxqtlib1::construct;
  new (return$) ::QJsonValue(qjsonvalue_init_from_qjsonarray$(value));
}

void rust$cxxqtlib1$cxxbridge1$205$qjsonvalue_init_from_qjsonobject(::QJsonObject const &value, ::QJsonValue *return$) noexcept {
  ::QJsonValue (*qjsonvalue_init_from_qjsonobject$)(::QJsonObject const &) = ::rust::cxxqtlib1::construct;
  new (return$) ::QJsonValue(qjsonvalue_init_from_qjsonobject$(value));
}

bool rust$cxxqtlib1$cxxbridge1$205$qjsonvalue_eq(::QJsonValue const &a, ::QJsonValue const &b) noexcept {
  bool (*qjsonvalue_eq$)(::QJsonValue const &, ::QJsonValue const &) = ::rust::cxxqtlib1::operatorEq;
  return qjsonvalue_eq$(a, b);
}

void rust$cxxqtlib1$cxxbridge1$205$qjsonvalue_to_debug_qstring(::QJsonValue const &value, ::QString *return$) noexcept {
  ::QString (*qjsonvalue_to_debug_qstring$)(::QJsonValue const &) = ::rust::cxxqtlib1::toDebugQString;
  new (return$) ::QString(qjsonvalue_to_debug_qstring$(value));
}
} // extern "C"
} // namespace cxxqtlib1
} // namespace rust
