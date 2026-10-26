import json
import os
import tempfile
import unittest
from pathlib import Path

from toyverse_diagnose import device


def fake_root(ram_mb, cores=2, driver=None, render=False, touch=False, temp=None,
              psi_full=0.0, psi_some=0.0, oom=0, battery=None, model="", efi=False):
    root = Path(tempfile.mkdtemp())
    (root / "proc/pressure").mkdir(parents=True)
    (root / "proc/cpuinfo").write_text("".join(f"processor\t: {i}\n" for i in range(cores)) + "Features\t: neon vfpv4\n")
    (root / "proc/meminfo").write_text(f"MemTotal:       {ram_mb * 1024} kB\n")
    (root / "proc/pressure/memory").write_text(
        f"some avg10=0.00 avg60={psi_some:.2f} avg300=0.00 total=0\nfull avg10=0.00 avg60={psi_full:.2f} avg300=0.00 total=0\n")
    (root / "proc/vmstat").write_text(f"nr_free_pages 1\noom_kill {oom}\n")
    if touch:
        (root / "proc/bus/input").mkdir(parents=True)
        (root / "proc/bus/input/devices").write_text('N: Name="Atmel maXTouch Touchscreen"\n')
    if driver:
        card = root / "sys/class/drm/card0/device"
        card.mkdir(parents=True)
        os.symlink(f"../../../bus/platform/drivers/{driver}", card / "driver")
    if render:
        (root / "dev/dri").mkdir(parents=True)
        (root / "dev/dri/renderD128").write_text("")
    if temp is not None:
        z = root / "sys/class/thermal/thermal_zone0"
        z.mkdir(parents=True)
        (z / "temp").write_text(str(int(temp * 1000)))
    if battery:
        b = root / "sys/class/power_supply/battery"
        b.mkdir(parents=True)
        (b / "type").write_text("Battery\n")
        (b / "status").write_text(battery + "\n")
    if model:
        (root / "proc/device-tree").mkdir(parents=True)
        (root / "proc/device-tree/model").write_text(model + "\x00")
    if efi:
        (root / "sys/firmware/efi").mkdir(parents=True)
    return root


class DeviceTests(unittest.TestCase):
    def test_galaxy_tab_2_is_minimal(self):
        root = fake_root(1000, driver="omapdrm", touch=True, model="Samsung Galaxy Tab 2 (10.1 inch)")
        hw = device.probe(root, arch="armv7l")
        self.assertEqual(device.base_class(hw), "K1")
        p = device.profile_for("K1", hw)
        self.assertEqual(p["oberflaeche"], "toytiles")
        self.assertEqual(p["browser"], "cog")
        self.assertEqual(p["animationen"], "aus")
        self.assertEqual(p["toywork"], "aus")
        self.assertEqual(p["apps"]["toyquest"]["modus"], "web")
        self.assertIn("armv7", device.install_path(hw)["weg"])

    def test_modern_pc_is_strong_and_may_run_full_node(self):
        root = fake_root(16384, cores=8, driver="amdgpu", render=True, efi=True)
        hw = device.probe(root, arch="x86_64")
        hw.disk_free_mb = 50_000
        self.assertEqual(device.base_class(hw), "K4")
        p = device.profile_for("K4", hw)
        self.assertEqual(p["toychain_stufe"], 3)
        self.assertEqual(p["apps"]["toyquest"]["modus"], "voll")
        self.assertIn("UEFI", device.install_path(hw)["weg"])

    def test_no_full_node_without_disk_space(self):
        hw = device.probe(fake_root(16384, driver="i915", render=True), arch="x86_64")
        hw.disk_free_mb = 500
        self.assertLess(device.profile_for("K4", hw)["toychain_stufe"], 3)

    def test_raspberry_pi_path(self):
        hw = device.probe(fake_root(4096, driver="v3d", render=True, model="Raspberry Pi 4 Model B Rev 1.4"), arch="aarch64")
        self.assertEqual(device.base_class(hw), "K3")
        self.assertIn("Pi", device.install_path(hw)["weg"])

    def test_32bit_x86_is_browser_only(self):
        hw = device.probe(fake_root(2048), arch="i686")
        self.assertIn("Kein eigenes", device.install_path(hw)["weg"])

    def test_on_battery_disables_toywork(self):
        hw = device.probe(fake_root(8192, driver="i915", render=True, battery="Discharging"), arch="x86_64")
        self.assertEqual(device.profile_for("K4", hw)["toywork"], "aus")

    def test_step_down_on_oom_and_up_after_calm_runs(self):
        base = dict(ram_mb=8192, driver="i915", render=True)
        state = {}
        hw = device.probe(fake_root(**base, oom=0), arch="x86_64")
        cls, state, _ = device.adapt(hw, state)
        self.assertEqual(cls, "K4")
        hw = device.probe(fake_root(**base, oom=2), arch="x86_64")
        cls, state, reasons = device.adapt(hw, state)
        self.assertEqual(cls, "K3")
        self.assertTrue(any("herunter" in r for r in reasons))
        for i in range(device.RELAX_RUNS_TO_STEP_UP - 1):
            cls, state, _ = device.adapt(device.probe(fake_root(**base, oom=2), arch="x86_64"), state)
            self.assertEqual(cls, "K3", f"calm run {i} must not step up yet")
        cls, state, reasons = device.adapt(device.probe(fake_root(**base, oom=2), arch="x86_64"), state)
        self.assertEqual(cls, "K4")

    def test_never_above_hardware_class(self):
        hw = device.probe(fake_root(1000, driver="omapdrm"), arch="armv7l")
        cls, _, _ = device.adapt(hw, {"klasse": "K4", "basis": "K1"})
        self.assertEqual(cls, "K1")

    def test_hot_device_disables_toywork(self):
        hw = device.probe(fake_root(8192, driver="i915", render=True, temp=82), arch="x86_64")
        _, _, reasons = device.adapt(hw, {})
        self.assertEqual(device.profile_for("K4", hw)["toywork"], "aus")
        self.assertTrue(any("Temperatur" in r for r in reasons))

    def test_run_writes_profile_for_launcher(self):
        root = fake_root(1000, driver="omapdrm", touch=True)
        out = Path(tempfile.mkdtemp())
        res = device.run(root, out / "state.json", out / "geraet.json", arch="armv7l")
        data = json.loads((out / "geraet.json").read_text())
        self.assertEqual(data["schema"], "toymesh-geraet/1")
        self.assertEqual(res["profil"]["klasse"], "K1")


if __name__ == "__main__":
    unittest.main()
