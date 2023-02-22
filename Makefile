# Rusttery
# @version 0.1.0

#files vars
BIN_CLIENT=rusttery
BIN_SERVER=rusttery-server
SERVICE=rusttery.service
DBUS_CONF=ar.claaj.Rusttery.conf

#dirs vars
BUILD_DIR=target/debug/
SERVICE_DIR=/usr/lib/systemd/system/
DBUS_DIR=/usr/share/dbus-1/system.d/
BIN_DIR=/usr/bin/

build:
	@cargo build

clean:
	@rm -r target

install:
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

reinstall: uninstall install
