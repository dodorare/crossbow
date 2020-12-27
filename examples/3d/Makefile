.PHONY: run install boot-sim generate clean

PROJECT_NAME = threed
DEVICE = ${DEVICE_ID}
ifndef DEVICE_ID
	DEVICE=$(shell xcrun simctl list devices 'iOS' | grep -v 'unavailable' | grep -v '^--' | grep -v '==' | head -n 1 | cut -d ' ' -f 7 | sed 's/[()]//g')
endif

run: build install run-sim
	xcrun simctl launch --console $(DEVICE) com.enfipy.$(PROJECT_NAME)

build:
	PROJECT_NAME=$(PROJECT_NAME) ./build.sh

run-sim:
	open -a Simulator --args -CurrentDeviceUDID $(DEVICE)

boot-sim:
	xcrun simctl boot $(DEVICE) || true

install: boot-sim
	cd ../../target/apple && xcrun simctl install $(DEVICE) $(PROJECT_NAME).app

clean:
	cargo clean
