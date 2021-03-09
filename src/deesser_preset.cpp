/*
 *  Copyright © 2017-2020 Wellington Wallace
 *
 *  This file is part of PulseEffects.
 *
 *  PulseEffects is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  PulseEffects is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with PulseEffects.  If not, see <https://www.gnu.org/licenses/>.
 */

#include "deesser_preset.hpp"

DeesserPreset::DeesserPreset()
    : input_settings(Gio::Settings::create("com.github.wwmm.pulseeffects.deesser",
                                           "/com/github/wwmm/pulseeffects/sourceoutputs/deesser/")),
      output_settings(Gio::Settings::create("com.github.wwmm.pulseeffects.deesser",
                                            "/com/github/wwmm/pulseeffects/sinkinputs/deesser/")) {}

void DeesserPreset::save(boost::property_tree::ptree& root,
                         const std::string& section,
                         const Glib::RefPtr<Gio::Settings>& settings) {
  root.put(section + ".deesser.state", settings->get_boolean("state"));

  root.put(section + ".deesser.detection", settings->get_string("detection"));

  root.put(section + ".deesser.mode", settings->get_string("mode"));

  root.put(section + ".deesser.threshold", settings->get_double("threshold"));

  root.put(section + ".deesser.ratio", settings->get_double("ratio"));

  root.put(section + ".deesser.laxity", settings->get_int("laxity"));

  root.put(section + ".deesser.makeup", settings->get_double("makeup"));

  root.put(section + ".deesser.f1-freq", settings->get_double("f1-freq"));

  root.put(section + ".deesser.f2-freq", settings->get_double("f2-freq"));

  root.put(section + ".deesser.f1-level", settings->get_double("f1-level"));

  root.put(section + ".deesser.f2-level", settings->get_double("f2-level"));

  root.put(section + ".deesser.f2-q", settings->get_double("f2-q"));

  root.put(section + ".deesser.sc-listen", settings->get_boolean("sc-listen"));
}

void DeesserPreset::load(const boost::property_tree::ptree& root,
                         const std::string& section,
                         const Glib::RefPtr<Gio::Settings>& settings) {
  update_key<bool>(root, settings, "state", section + ".deesser.state");

  update_string_key(root, settings, "detection", section + ".deesser.detection");

  update_string_key(root, settings, "mode", section + ".deesser.mode");

  update_key<double>(root, settings, "threshold", section + ".deesser.threshold");

  update_key<double>(root, settings, "ratio", section + ".deesser.ratio");

  update_key<int>(root, settings, "laxity", section + ".deesser.laxity");

  update_key<double>(root, settings, "makeup", section + ".deesser.makeup");

  update_key<double>(root, settings, "f1-freq", section + ".deesser.f1-freq");

  update_key<double>(root, settings, "f2-freq", section + ".deesser.f2-freq");

  update_key<double>(root, settings, "f1-level", section + ".deesser.f1-level");

  update_key<double>(root, settings, "f2-level", section + ".deesser.f2-level");

  update_key<double>(root, settings, "f2-q", section + ".deesser.f2-q");

  update_key<bool>(root, settings, "sc-listen", section + ".deesser.sc-listen");
}

void DeesserPreset::write(PresetType preset_type, boost::property_tree::ptree& root) {
  switch (preset_type) {
    case PresetType::output:
      save(root, "output", output_settings);
      break;
    case PresetType::input:
      save(root, "input", input_settings);
      break;
  }
}

void DeesserPreset::read(PresetType preset_type, const boost::property_tree::ptree& root) {
  switch (preset_type) {
    case PresetType::output:
      load(root, "output", output_settings);
      break;
    case PresetType::input:
      load(root, "input", input_settings);
      break;
  }
}