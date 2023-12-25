#!/bin/bash -e

if [ "$(id -u)" != "0" ]; then
   echo "This script must be run as root" 1>&2
   exit 1
fi

DBUS_BIN=msi_ec_dbus
DBUS_CONF='msi_ec_dbus.conf'
DBUS_SERVICE='org.msi_ec_dbus.service'

LIBEXEC_PATH='/usr/libexec/'
DBUS_SYSTEM_PATH='/usr/share/dbus-1/system.d/'
DBUS_SERVICES_PATH='/usr/share/dbus-1/system-services/'

GUI_BIN=msi_control

echo "Installation..."

mkdir -p /opt/msi_control
cp ./msi_control /opt/msi_control/msi_control
cp -r ./icons /opt/msi_control/

install -vDm755 ./$DBUS_BIN $LIBEXEC_PATH$DBUS_BIN
install -vDm644 ./dbus_conf/$DBUS_CONF $DBUS_SYSTEM_PATH$DBUS_CONF
install -vDm644 ./dbus_conf/$DBUS_SERVICE $DBUS_SERVICES_PATH$DBUS_SERVICE

echo "Done."
