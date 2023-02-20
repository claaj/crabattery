#!/usr/bin/env sh
#Setup for Rusttery

#files/path vars
build=rusttery
build_server=rusttery-server
service=rusttery.service
dbus_conf=ar.claaj.Rusttery.conf

#dirs vars
build_dir=target/debug/
service_dir=/usr/lib/systemd/system/
dbus_dir=/usr/share/dbus-1/system.d/
bin_dir=/usr/bin/

if test $# -eq 1
then
   if [[ "$1" == "--install" ]]
   then
      cargo build
      sudo cp resources/$service $service_dir
      sudo cp resources/$dbus_conf $dbus_dir
      sudo cp $build_dir$build $bin_dir
      sudo cp $build_dir$build_server $bin_dir
      sudo systemctl enable --now $service
   elif [[ "$1" == "--remove" ]]
   then
       sudo systemctl disable --now $service
       sudo rm -i $service_dir$service
       sudo rm -i $dbus_dir$dbus_conf
       sudo rm -i $bin_dir$build
       sudo rm -i $bin_dir$build_server
   else
       echo 'Invalid argument'
   fi
else
    echo 'Invalid ammount of arguments'
fi
