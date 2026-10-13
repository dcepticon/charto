#include "cxx-qt-lib/qmetatype.h"
#include <cstdint>
#include <type_traits>

namespace rust {
  namespace cxxqtlib1 {
    using QMetaTypeType = ::rust::cxxqtlib1::QMetaTypeType;
  }
}

namespace rust {
namespace cxxqtlib1 {
static_assert(::std::is_enum<QMetaTypeType>::value, "expected enum");
static_assert(sizeof(QMetaTypeType) == sizeof(::std::int32_t), "incorrect size");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::UnknownType) == 0, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::Bool) == 1, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::Int) == 2, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::UInt) == 3, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::LongLong) == 4, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::ULongLong) == 5, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::Double) == 6, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::Long) == 32, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::Short) == 33, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::Char) == 34, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::ULong) == 35, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::UShort) == 36, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::UChar) == 37, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::Float) == 38, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::VoidStar) == 31, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QChar) == 7, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QString) == 10, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QStringList) == 11, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QByteArray) == 12, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QBitArray) == 13, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QDate) == 14, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QTime) == 15, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QDateTime) == 16, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QUrl) == 17, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QLocale) == 18, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QRect) == 19, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QRectF) == 20, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QSize) == 21, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QSizeF) == 22, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QLine) == 23, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QLineF) == 24, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QPoint) == 25, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QPointF) == 26, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QEasingCurve) == 29, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QUuid) == 30, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QVariant) == 41, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QModelIndex) == 42, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QPersistentModelIndex) == 50, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QRegularExpression) == 44, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QJsonValue) == 45, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QJsonObject) == 46, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QJsonArray) == 47, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QJsonDocument) == 48, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QByteArrayList) == 49, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QObjectStar) == 39, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::SChar) == 40, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::Void) == 43, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::Nullptr) == 51, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QVariantMap) == 8, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QVariantList) == 9, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QVariantHash) == 28, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QCborSimpleType) == 52, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QCborValue) == 53, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QCborArray) == 54, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QCborMap) == 55, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::Char16) == 56, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::Char32) == 57, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QFont) == 4096, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QPixmap) == 4097, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QBrush) == 4098, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QColor) == 4099, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QPalette) == 4100, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QIcon) == 4101, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QImage) == 4102, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QPolygon) == 4103, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QRegion) == 4104, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QBitmap) == 4105, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QCursor) == 4106, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QKeySequence) == 4107, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QPen) == 4108, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QTextLength) == 4109, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QTextFormat) == 4110, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QTransform) == 4112, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QMatrix4x4) == 4113, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QVector2D) == 4114, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QVector3D) == 4115, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QVector4D) == 4116, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QQuaternion) == 4117, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QPolygonF) == 4118, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QColorSpace) == 4119, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::QSizePolicy) == 8192, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::int32_t>(QMetaTypeType::User) == 65536, "disagrees with the value in #[cxx::bridge]");
} // namespace cxxqtlib1
} // namespace rust
