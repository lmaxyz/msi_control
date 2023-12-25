#!/bin/bash -e

APP_BIN=msi_ec_backend
DBUS_CONF='msi_ec_backend.conf'
DBUS_SERVICE='org.msi_ec_backend.service'

LIBEXEC_PATH='/usr/libexec/'
DBUS_SYSTEM_PATH='/usr/share/dbus-1/system.d/'
DBUS_SERVICES_PATH='/usr/share/dbus-1/system-services/'

echo "Installation..."

mkdir -p /opt/msi-control
cp ./msi-control /opt/msi-control/msi-control
cp ./requirements.txt /opt/msi-control/
cp -r ./icons /opt/msi-control/
python3 -m venv /opt/msi-control/.venv
/opt/msi-control/.venv/bin/pip install -r /opt/msi-control/requirements.txt

install -vDm755 ./$APP_BIN $LIBEXEC_PATH$APP_BIN
install -vDm644 ./msi_ec_backend/dbus_conf/$DBUS_CONF $DBUS_SYSTEM_PATH$DBUS_CONF
install -vDm644 ./msi_ec_backend/dbus_conf/$DBUS_SERVICE $DBUS_SERVICES_PATH$DBUS_SERVICE

echo "Done."
