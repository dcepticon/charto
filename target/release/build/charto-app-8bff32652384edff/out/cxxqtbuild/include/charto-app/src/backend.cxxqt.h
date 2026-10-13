#pragma once

#include <cxx-qt/casting.h>
#include <cxx-qt/signalhandler.h>
#include <cxx-qt/threading.h>
#include <cxx-qt/type.h>

class ChartBackend;
using ChartBackendCxxQtThread = ::rust::cxxqt1::CxxQtThread<ChartBackend>;

namespace rust::cxxqtgen1 {
using ChartBackendCxxQtSignalHandlersnapshotChanged = ::rust::cxxqt1::SignalHandler<struct ChartBackendCxxQtSignalParamssnapshotChanged *>;
} // namespace rust::cxxqtgen1

namespace rust::cxxqtgen1 {
using ChartBackendCxxQtSignalHandlerstatusChanged = ::rust::cxxqt1::SignalHandler<struct ChartBackendCxxQtSignalParamsstatusChanged *>;
} // namespace rust::cxxqtgen1

namespace rust::cxxqtgen1 {
using ChartBackendCxxQtSignalHandlerpickedChanged = ::rust::cxxqt1::SignalHandler<struct ChartBackendCxxQtSignalParamspickedChanged *>;
} // namespace rust::cxxqtgen1


#include "charto-app/src/backend.cxx.h"


namespace rust::cxxqtgen1 {
::QMetaObject::Connection
ChartBackend_snapshotChangedConnect(ChartBackend& self, ::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlersnapshotChanged closure, ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1

namespace rust::cxxqtgen1 {
::QMetaObject::Connection
ChartBackend_statusChangedConnect(ChartBackend& self, ::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerstatusChanged closure, ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1

namespace rust::cxxqtgen1 {
::QMetaObject::Connection
ChartBackend_pickedChangedConnect(ChartBackend& self, ::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerpickedChanged closure, ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1

class ChartBackend : public QObject, private ::rust::cxxqt1::CxxQtType<ChartBackendRust>, private ::rust::cxxqt1::CxxQtThreading<ChartBackend>
{
  Q_OBJECT
public:
  QML_NAMED_ELEMENT(ChartBackend)
  Q_PROPERTY(QString snapshot READ getSnapshot WRITE setSnapshot NOTIFY snapshotChanged)
  Q_PROPERTY(QString status READ getStatus WRITE setStatus NOTIFY statusChanged)
  Q_PROPERTY(QString picked READ getPicked WRITE setPicked NOTIFY pickedChanged)

  virtual ~ChartBackend() = default;

public:
  QString const& getSnapshot() const noexcept;
  Q_SLOT void setSnapshot(QString value) noexcept;
  QString const& getStatus() const noexcept;
  Q_SLOT void setStatus(QString value) noexcept;
  QString const& getPicked() const noexcept;
  Q_SLOT void setPicked(QString value) noexcept;
  Q_SIGNAL void snapshotChanged();
  Q_SIGNAL void statusChanged();
  Q_SIGNAL void pickedChanged();
  Q_INVOKABLE QString paletteJson() const noexcept;
  Q_INVOKABLE void newChart(::std::int32_t rows, ::std::int32_t cols) noexcept;
  Q_INVOKABLE void openFile(QString const& path) noexcept;
  Q_INVOKABLE void saveFile(QString const& path) noexcept;
  Q_INVOKABLE void setActiveSymbol(QString const& name) noexcept;
  Q_INVOKABLE void setColor(QString const& hex) noexcept;
  Q_INVOKABLE void addColor(QString const& hex) noexcept;
  Q_INVOKABLE void setMode(::std::int32_t mode) noexcept;
  Q_INVOKABLE void clickCell(::std::int32_t row, ::std::int32_t col) noexcept;
  Q_INVOKABLE void selectRect(::std::int32_t col, ::std::int32_t row, ::std::int32_t num_cols, ::std::int32_t num_rows) noexcept;
  Q_INVOKABLE void selectRow(::std::int32_t row) noexcept;
  Q_INVOKABLE void selectColumn(::std::int32_t col) noexcept;
  Q_INVOKABLE void clearSelection() noexcept;
  Q_INVOKABLE void applyColor() noexcept;
  Q_INVOKABLE void undo() noexcept;
  Q_INVOKABLE void redo() noexcept;
  Q_INVOKABLE void copy() noexcept;
  Q_INVOKABLE void paste(::std::int32_t col, ::std::int32_t row) noexcept;
  Q_INVOKABLE void insertRows(::std::int32_t num, bool above) noexcept;
  Q_INVOKABLE void deleteRows() noexcept;
  Q_INVOKABLE void insertColumns(::std::int32_t num, bool left) noexcept;
  Q_INVOKABLE void deleteColumns() noexcept;
  Q_INVOKABLE void addRowRepeat(::std::int32_t multiplicity) noexcept;
  Q_INVOKABLE void deleteRowRepeat() noexcept;
  Q_INVOKABLE bool hasRecovery(QString const& path) const noexcept;
  Q_INVOKABLE void openRecovery(QString const& path) noexcept;
  Q_INVOKABLE void autosave() noexcept;
  Q_INVOKABLE void endSession() noexcept;
  Q_INVOKABLE void clearRecentFiles() noexcept;
  Q_INVOKABLE void selectSameColor(::std::int32_t row, ::std::int32_t col) noexcept;
  Q_INVOKABLE void selectSameSymbol(::std::int32_t row, ::std::int32_t col) noexcept;
  Q_INVOKABLE void setRowLabel(::std::int32_t row, QString const& text) noexcept;
  Q_INVOKABLE void setColumnLabel(::std::int32_t col, QString const& text) noexcept;
  Q_INVOKABLE QString settingsJson() const noexcept;
  Q_INVOKABLE void applySettings(QString const& json) noexcept;
  Q_INVOKABLE void exportSvg(QString const& path) noexcept;
  Q_INVOKABLE QString printTempPath() const noexcept;
  Q_INVOKABLE void printImage(QString const& png_path, QString const& title) noexcept;
  Q_INVOKABLE void chooseFile(QString const& purpose, QString const& current) noexcept;
  Q_INVOKABLE void reportStatus(QString const& message) noexcept;
  explicit ChartBackend(QObject* parent = nullptr);

private:
  template<typename T>
friend auto ::rust::cxxqt1::qtThread(const T& qobject) -> ::rust::cxxqt1::CxxQtThread<T>;


private:
  template<typename Inner, typename Outer>
  friend Inner& ::rust::cxxqt1::unsafeRustMut(Outer& outer);

  template<typename Inner, typename Outer>
  friend const Inner& ::rust::cxxqt1::unsafeRust(const Outer& outer);
};

static_assert(::std::is_base_of<QObject, ChartBackend>::value, "ChartBackend must inherit from QObject");


Q_DECLARE_METATYPE(ChartBackend*)

