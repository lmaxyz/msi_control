#!/bin/bash -e

cd ../
mkdir -p build
cp -r ./msi_ec_dbus/dbus_conf ./build/dbus_conf
cp -r ./msi_control_ui/icons ./build/icons
cp ./scripts/install.sh ./build
cp ./scripts/uninstall.sh ./build

echo 'Building dbus service...'
cd ./msi_ec_dbus
cargo build --release
cp ./target/release/msi_ec_dbus ../build/

echo 'Building binary...'
cd ../msi_control_ui
cargo build --release
cp ./target/release/msi_control ../build/

echo 'Done.'
