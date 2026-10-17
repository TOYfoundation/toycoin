## Global VFX settings: quality and photosensitivity protection.
##
## Flash safety (toypay/docs/07_SPIELDESIGN.md, section 7):
## - At most 3 flashes per second (WCAG 2.3.1 general flash threshold).
## - Player setting "Blitze und Flackern": NORMAL / SOFT / OFF.
##   SOFT turns lightning into a gentle glow, OFF removes flashes entirely.
## Every effect in this folder asks request_flash() before it flashes.
class_name VfxSettings
extends RefCounted

enum FlashMode { NORMAL, SOFT, OFF }
enum Quality { LOW, MEDIUM, HIGH }

const MAX_FLASHES_PER_SECOND := 3
const FLASH_WINDOW_MS := 1000

static var flash_mode: FlashMode = FlashMode.SOFT  # safe default until the player chooses
static var quality: Quality = Quality.MEDIUM
static var _flash_times: Array[int] = []


## Particle budget per effect: LOW (phones, Raspberry Pi) <= 64, HIGH <= 512.
static func particle_amount(base: int) -> int:
	match quality:
		Quality.LOW:
			return clampi(base / 4, 4, 64)
		Quality.MEDIUM:
			return clampi(base / 2, 8, 256)
		_:
			return clampi(base, 8, 512)


## Lightning geometry is skipped on LOW quality and when flashes are off.
static func lightning_allowed() -> bool:
	return quality != Quality.LOW and flash_mode != FlashMode.OFF


## Returns true if a bright flash may be shown now and records it.
## Returns false in SOFT/OFF mode or if 3 flashes already happened within
## the last second. Callers then fall back to a soft glow.
static func request_flash(now_ms: int = -1) -> bool:
	if flash_mode != FlashMode.NORMAL:
		return false
	var now: int = now_ms if now_ms >= 0 else Time.get_ticks_msec()
	while not _flash_times.is_empty() and now - _flash_times[0] >= FLASH_WINDOW_MS:
		_flash_times.pop_front()
	if _flash_times.size() >= MAX_FLASHES_PER_SECOND:
		return false
	_flash_times.append(now)
	return true


## Brightness multiplier for glow-type effects in the current mode.
static func flash_intensity() -> float:
	match flash_mode:
		FlashMode.NORMAL:
			return 1.0
		FlashMode.SOFT:
			return 0.45
		_:
			return 0.25


static func reset_flash_history() -> void:
	_flash_times.clear()
