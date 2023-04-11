# Crabattery
# @version 0.1.0

#files vars
BIN_CLIENT=crabattery
BIN_SERVER=crabattery-server
SERVICE=crabattery.service
DBUS_CONF=ar.claaj.Crabattery.conf
LIMIT_FILE=crabattery.limit

#dirs vars
BUILD_DIR=target/debug/
SERVICE_DIR=/usr/lib/systemd/system/
DBUS_DIR=/usr/share/dbus-1/system.d/
BIN_DIR=/usr/bin/
LIMIT_DIR=/etc/crabattery/

build:
	@cargo build

clean:
	@rm -r target

install:
	@mkdir -p $(LIMIT_DIR)
	@cp resources/$(LIMIT_FILE) $(LIMIT_DIR)
	@cp resources/$(SERVICE) $(SERVICE_DIR)
	@cp resources/$(DBUS_CONF) $(DBUS_DIR)
	@cp $(BUILD_DIR)$(BIN_CLIENT) $(BIN_DIR)
	@cp $(BUILD_DIR)$(BIN_SERVER) $(BIN_DIR)
	@systemctl enable --now $(SERVICE)

uninstall:
	@systemctl disable --now $(SERVICE)
	@rm -f $(SERVICE_DIR)$(SERVICE)
	@rm -f $(DBUS_DIR)$(DBUS_CONF)
	@rm -f $(BIN_DIR)$(BIN_CLIENT)
	@rm -f $(BIN_DIR)$(BIN_SERVER)
	@rm -fr $(LIMIT_DIR)

reinstall: uninstall install
