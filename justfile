# Crabattery 0.1.0

build:
    @cargo build --release

clean:
    @rm -r target

install:
    @mkdir -p /etc/crabattery/
    @cp resources/crabattery.limit /etc/crabattery/
    @cp resources/crabattery.service /usr/lib/systemd/system/
    @cp resources/ar.claaj.Crabattery.conf /usr/share/dbus-1/system.d/
    @cp target/release/crabattery /usr/bin/
    @cp target/release/crabattery-server /usr/bin/
    @systemctl enable --now crabattery

uninstall:
    @systemctl disable --now crabattery
    @rm -f /usr/lib/systemd/system/crabattery.service
    @rm -f /usr/share/dbus-1/system.d/ar.claaj.Crabattery.conf
    @rm -f /usr/bin/crabattery
    @rm -f /usr/bin/crabattery-server
    @rm -fr /etc/crabattery/

reinstall: uninstall install
