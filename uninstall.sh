#!/bin/bash -e

echo 'Uninstallation...'

rm -rfv /opt/msi-control

rm -fv /usr/libexec/msi_ec_backend
rm -fv /usr/share/dbus-1/system.d/msi_ec_backend.conf
rm -fv /usr/share/dbus-1/system-services/org.msi_ec_backend.service

echo 'Done.'
