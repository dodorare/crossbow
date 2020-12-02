.PHONY: run install boot-sim generate clean

DEVICE = ${DEVICE_ID}
ifndef DEVICE_ID
	DEVICE=$(shell xcrun simctl list devices 'iOS' | grep -v 'unavailable' | grep -v '^--' | grep -v '==' | head -n 1 | cut -d ' ' -f 7 | sed 's/[()]//g')
endif

run: build install run-sim
	xcrun simctl launch --console $(DEVICE) com.rust.threed

build:
	./build.sh

run-sim:
	open -a Simulator --args -CurrentDeviceUDID $(DEVICE_ID)

boot-sim:
	xcrun simctl boot $(DEVICE) || true

install: boot-sim
	cd ../../target/apple && xcrun simctl install $(DEVICE) threed.app

clean:
	cargo clean
