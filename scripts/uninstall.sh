#!/bin/bash -e

if [ "$(id -u)" != "0" ]; then
   echo "This script must be run as root" 1>&2
   exit 1
fi

echo 'Uninstallation...'

rm -rfv /opt/msi_control

rm -fv /usr/libexec/msi_ec_dbus
rm -fv /usr/share/dbus-1/system.d/msi_ec_dbus.conf
rm -fv /usr/share/dbus-1/system-services/org.msi_ec_dbus.service

echo 'Done.'
