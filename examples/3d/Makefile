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
	open -a Simulator --args -CurrentDeviceUDID C9133F14-AF02-4284-B8E8-D07CF75FC8DD

boot-sim:
	xcrun simctl boot C9133F14-AF02-4284-B8E8-D07CF75FC8DD || true

install: boot-sim
	cd ../../target/apple && xcrun simctl install C9133F14-AF02-4284-B8E8-D07CF75FC8DD example.app

clean:
	cargo clean
