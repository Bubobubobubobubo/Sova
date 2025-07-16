# Analyse des utilisations de print/println dans BuboCore

## Dossier core/

### Utilisations de print/println dans core/

**src/scene.rs**
- Ligne 107: `eprintln!(`
- Ligne 126: `eprintln!(`

**src/lang.rs**
- Ligne 57: `pub fn debug_print(prog: &Program, about: String, begin: String) {`
- Ligne 59: `println!("{}BEGIN: {}", begin, info);`
- Ligne 69: `println!(`
- Ligne 84: `println!("{}{}: {:?} ➡️  {}", begin, count, inst, x)`
- Ligne 90: `println!("{}{}: Control(Mov(", begin, count);`
- Ligne 91: `debug_print(f, "FUNCTION".to_string(), "   ".to_string());`
- Ligne 92: `println!("{}   {:?}))", begin, f_content);`
- Ligne 94: `_ => println!("{}{}: {:?}", begin, count, inst),`
- Ligne 97: `println!("{}END: {}", begin, info);`

**src/relay_client.rs**
- Ligne 109: `println!("[RELAY] Connecting to {}...", self.config.relay_address);`
- Ligne 133: `println!("[RELAY] Connected successfully! Instance ID: {:?}", assigned_id);`
- Ligne 134: `println!("[RELAY] Other instances: {:?}", current_instances.iter().map(|i| &i.name).collect::<Vec<_>>());`
- Ligne 165: `eprintln!("[RELAY] Read error: {}", e);`
- Ligne 178: `eprintln!("[RELAY] Write error: {}", e);`
- Ligne 186: `eprintln!("[RELAY] Writer task exited");`
- Ligne 237: `eprintln!("[RELAY] Connection lost, marking as disconnected");`

**src/device_map.rs**
- Ligne 74: `println!("[+] MIDI Input initialized successfully.");`
- Ligne 78: `eprintln!("[!] Failed to initialize MIDI Input: {}", e);`
- Ligne 85: `println!("[+] MIDI Output initialized successfully.");`
- Ligne 89: `eprintln!("[!] Failed to initialize MIDI Output: {}", e);`
- Ligne 159: `println!("[+] Assigned device '{}' to Slot {}", device_name, slot_id);`
- Ligne 165: `/// If the slot was already empty, it prints a message indicating so.`
- Ligne 182: `println!(`
- Ligne 187: `println!("[~] Slot {} was already empty.", slot_id);`
- Ligne 194: `/// If the device was assigned to a slot, it removes the assignment and prints a message.`
- Ligne 208: `println!("[-] Unassigned device '{}' from Slot {}", device_name, slot);`
- Ligne 618: `eprintln!(`
- Ligne 702: `println!("[~] Generating device list (excluding implicit log)...");`
- Ligne 817: `println!("[~] Device list generated. Count: {}", final_list.len());`
- Ligne 834: `println!(`
- Ligne 859: `println!("[✅] Connected MIDI Input: {}", device_name);`
- Ligne 863: `println!("[✅] Connected MIDI Output: {}", device_name);`
- Ligne 871: `println!("[✅] Registered MIDI device: {}", device_name);`
- Ligne 877: `eprintln!(`
- Ligne 890: `eprintln!(`
- Ligne 916: `println!(`
- Ligne 942: `println!(`
- Ligne 954: `eprintln!(`
- Ligne 965: `eprintln!(`
- Ligne 976: `eprintln!(`
- Ligne 1004: `println!(`
- Ligne 1023: `println!(`
- Ligne 1031: `println!(`
- Ligne 1048: `println!("[✅] Registered virtual MIDI port pair: '{}'", desired_name);`
- Ligne 1053: `eprintln!(`
- Ligne 1066: `eprintln!(`
- Ligne 1098: `println!(`
- Ligne 1116: `eprintln!("[!] {}", err_msg);`
- Ligne 1130: `eprintln!("[!] {}", err_msg);`
- Ligne 1148: `println!(`
- Ligne 1154: `println!("[✅] Registered OSC Output device: '{}'", name);`
- Ligne 1162: `eprintln!("[!] {}", err_msg);`
- Ligne 1181: `println!("[🗑️] Removing OSC Output device: '{}'", name);`
- Ligne 1195: `println!("[✅] Removed OSC Output device registration: '{}'", name);`
- Ligne 1204: `eprintln!("[!] {}", err_msg);`
- Ligne 1213: `eprintln!("[!] {}", err_msg);`
- Ligne 1224: `println!("[!] Sending MIDI Panic (All Notes Off CC 123) to all outputs...");`
- Ligne 1230: `println!("[!] Sending Panic to MIDI device: {}", name);`
- Ligne 1243: `eprintln!(`
- Ligne 1260: `eprintln!("[!] Could not lock Mutex for MIDI device: {}", name);`
- Ligne 1264: `println!("[!] MIDI Panic finished.");`

**deprecated_examples/replacing_scripts.rs**
- Ligne 72: `println!("Adding line 2");`
- Ligne 76: `println!("Adding line 3");`
- Ligne 82: `println!("Removing line 0");`
- Ligne 86: `println!("Removing line 1");`
- Ligne 90: `println!("Removing line 2");`

**src/schedule/scheduler_actions.rs**
- Ligne 167: `eprintln!(`
- Ligne 184: `eprintln!(`
- Ligne 213: `eprintln!(`
- Ligne 230: `eprintln!(`
- Ligne 272: `eprintln!(`
- Ligne 290: `eprintln!(`
- Ligne 307: `eprintln!(`
- Ligne 328: `eprintln!(`
- Ligne 366: `eprintln!(`
- Ligne 408: `eprintln!(`
- Ligne 427: `eprintln!(`
- Ligne 442: `eprintln!(`
- Ligne 453: `eprintln!(`
- Ligne 506: `eprintln!(`
- Ligne 530: `eprintln!(`
- Ligne 552: `eprintln!(`
- Ligne 558: `eprintln!(`
- Ligne 578: `eprintln!(`
- Ligne 584: `eprintln!(`

**src/scene/script.rs**
- Ligne 173: `//print!("Executing this instruction: {:?}\n", current);`

**deprecated_examples/bali_test_prog.rs**
- Ligne 37: `print!("{}", GREETER_LOGO);`
- Ligne 38: `println!("Version: {}\n", env!("CARGO_PKG_VERSION"));`
- Ligne 131: `eprintln!("[!] Failed to send initial scene to scheduler: {}", e);`
- Ligne 151: `println!(`
- Ligne 160: `eprintln!(`
- Ligne 164: `eprintln!(`
- Ligne 169: `// For other errors, print a generic message and the error details`
- Ligne 170: `eprintln!("[!] Server failed to start: {}", e);`
- Ligne 176: `println!("\n[-] Stopping BuboCore...");`
- Ligne 187: `print!("Plop\n");`
- Ligne 218: `println!("{:?}", msg);`

**deprecated_examples/dummy_client.rs**
- Ligne 68: `println!("\n[-] Stopping BuboCore...");`
- Ligne 103: `println!("{:?}", msg);`

**src/world.rs**
- Ligne 96: `println!("[+] Starting world at {start_date}");`
- Ligne 127: `println!("[-] Exiting world...");`
- Ligne 137: `println!("[-] World received shutdown signal");`
- Ligne 187: `println!(`

**src/schedule/playback.rs**
- Ligne 54: `println!(`
- Ligne 67: `println!(`
- Ligne 85: `println!(`
- Ligne 101: `println!(`
- Ligne 138: `println!(`
- Ligne 155: `println!("[SCHEDULER] Requesting transport stop via Link now");`
- Ligne 194: `println!(`

**src/compiler.rs**
- Ligne 182: `// eprintln!("Failed to load syntax file for {}: {}", self.name(), e);`

**src/scene/line.rs**
- Ligne 287: `/// Although the code prints an error, it does not panic if \`position > self.frames.len()\`.`
- Ligne 292: `eprintln!("[!] Frame::insert_frame: Invalid position {}", position);`
- Ligne 325: `/// Although the code prints an error, it does not panic if \`position >= self.frames.len()\`.`
- Ligne 329: `eprintln!("[!] Frame::remove_frame: Invalid position {}", position);`
- Ligne 334: `println!(`
- Ligne 346: `println!(`
- Ligne 507: `/// If \`frame_index\` is out of bounds, an error is printed to stderr and no change is made.`
- Ligne 512: `eprintln!("[!] Line::set_frame_name: Invalid index {}", frame_index);`

**src/schedule.rs**
- Ligne 184: `println!("[-] Scheduler received shutdown signal");`
- Ligne 243: `println!(`
- Ligne 253: `println!("[+] Starting scheduler at {start_date}");`
- Ligne 305: `println!("Applying deferred action: {:?}", action); // Debug log`
- Ligne 422: `println!("[-] Exiting scheduler...");`

**src/compiler/bali/bali_compiler.rs**
- Ligne 4: `use crate::lang::{Program, debug_print};`
- Ligne 27: `// print program for debug`
- Ligne 29: `debug_print(&res, "PROGRAM".to_string(), "".to_string());`

**src/lang/event.rs**
- Ligne 186: `eprintln!(`
- Ligne 266: `eprintln!(`

**src/protocol/midi.rs**
- Ligne 588: `println!(`
- Ligne 601: `println!(`
- Ligne 648: `println!("[MIDI IN VIRTUAL RAW] Port: {}, Data: {:?}", connection_name_clone, message);`
- Ligne 656: `println!("[MIDI IN VIRTUAL] CC Received - Port: {}, Channel: {}, Control: {}, Value: {}", connection_name_clone, channel, control, value);`

**src/main.rs**
- Ligne 54: `print!("{}", GREETER_LOGO);`
- Ligne 55: `println!("Version: {}\n", env!("CARGO_PKG_VERSION"));`
- Ligne 67: `println!("[+] Initializing Sova audio engine...");`
- Ligne 77: `println!(`
- Ligne 94: `println!(`
- Ligne 150: `eprintln!("Failed to create OSC server: {}", e);`
- Ligne 158: `println!("   Audio engine ready ✓");`
- Ligne 275: `eprintln!(`
- Ligne 280: `println!(`
- Ligne 286: `eprintln!("[!] Failed to assign '{}' to Slot 1: {}", midi_name, e);`
- Ligne 295: `eprintln!(`
- Ligne 300: `println!(`
- Ligne 306: `eprintln!("[!] Failed to assign '{}' to Slot 2: {}", osc_name, e);`
- Ligne 421: `eprintln!("[!] Failed to send initial scene to scheduler: {}", e);`
- Ligne 428: `println!("[+] Initializing relay client...");`
- Ligne 440: `println!("[+] Connected to relay server at {}", relay_addr);`
- Ligne 441: `println!("[+] Relay client instance ID: {:?}", client.instance_id());`
- Ligne 445: `eprintln!("[!] Failed to connect to relay server: {}", e);`
- Ligne 446: `eprintln!("    Continuing in local mode...");`
- Ligne 472: `println!("[RELAY] Starting relay message handler task");`
- Ligne 482: `eprintln!("[RELAY] Connection lost, relay handler exiting");`
- Ligne 494: `println!("[RELAY] Received update from instance '{}': {:?}", source_instance_name, client_msg);`
- Ligne 508: `eprintln!("[RELAY] Failed to apply SetScript: {}", e);`
- Ligne 525: `println!("[RELAY] Unhandled message type from remote instance");`
- Ligne 530: `eprintln!("[RELAY] Failed to deserialize client message: {}", e);`
- Ligne 535: `println!("[RELAY] Instance '{}' disconnected", instance_name);`
- Ligne 544: `eprintln!("[RELAY] Relay message channel closed, handler exiting");`
- Ligne 553: `println!(`
- Ligne 562: `eprintln!(`
- Ligne 566: `eprintln!(`
- Ligne 571: `// For other errors, print a generic message and the error details`
- Ligne 572: `eprintln!("[!] Server failed to start: {}", e);`

**src/compiler/bali/bali_ast/expression.rs**
- Ligne 112: `println!("Call function {} with args {:?}", name, args);`

**src/lang/control_asm.rs**
- Ligne 359: `eprintln!("[!] Runtime Error: Pop from empty stack into Var {:?}", x);`
- Ligne 380: `eprintln!(`
- Ligne 866: `// Optional Debug: println!("[VM GetMidiCC] Resolved Dev: {}, Chan: {}, Ctrl: {}, Result: {}", device_id, channel_val, control_val, cc_value);`
- Ligne 868: `eprintln!(`
- Ligne 874: `eprintln!(`
- Ligne 880: `eprintln!(`
- Ligne 886: `eprintln!(`
- Ligne 893: `eprintln!(`

**src/compiler/bali/bali_ast.rs**
- Ligne 68: `//print!("Original prog {:?}\n", prog);`
- Ligne 70: `//print!("Loopless prog {:?}\n", prog);`
- Ligne 94: `println!("Function {}: {:?}", func_name, func_content);`
- Ligne 144: `//print!("Choice variables {:?}\n", choice_variables);`
- Ligne 145: `//print!("Pick variables {:?}\n", pick_variables);`
- Ligne 148: `println!("BEGIN: {}", info);`
- Ligne 150: `println!("{:?}", ts);`
- Ligne 152: `println!("END: {}", info);`
- Ligne 155: `//print!("Sorted prog {:?}\n", prog);`
- Ligne 174: `//print!("{:?}\n", prog[i]);`
- Ligne 197: `//print!("NEW TIME STATEMENT!\n");`
- Ligne 209: `// print program for debug`
- Ligne 213: `print!("BEGIN: {}\n", info);`
- Ligne 223: `print!("{}: {:?} ➡️  {}\n", count, inst, count + x)`
- Ligne 232: `print!("{}: {:?} ➡️  {}\n", count, inst, count + x)`
- [Suite tronquée...]

---

## Dossier engine/

### Utilisations de print/println dans engine/

**src/engine.rs**
- Ligne 22: `macro_rules! rt_eprintln {`
- Ligne 27: `macro_rules! rt_eprintln {`
- Ligne 29: `eprintln!($($arg)*);`
- Ligne 766: `Ok(_) => println!(`
- Ligne 771: `eprintln!(`
- Ligne 775: `eprintln!(`
- Ligne 782: `eprintln!(`
- Ligne 789: `println!("Audio thread real-time priority disabled (priority = 0)");`
- Ligne 799: `println!(`
- Ligne 807: `println!("Audio device fallback: {}", reason);`
- Ligne 811: `eprintln!("Failed to select audio device: {}", err);`
- Ligne 892: `println!(`
- Ligne 994: `rt_eprintln!(`

**src/device_selector.rs**
- Ligne 32: `println!("Audio device selection starting...");`
- Ligne 42: `println!("  Trying strategy {}: {}", i + 1, strategy.name());`
- Ligne 49: `println!("  Found device: {}", device_name);`
- Ligne 66: `println!("  Device validation failed for: {}", device_name);`
- Ligne 70: `println!("  No device found with this strategy");`
- Ligne 88: `println!("    No compatible configuration found");`
- Ligne 95: `println!("    Failed to query device configurations: {}", e);`
- Ligne 198: `println!("    Trying platform-specific device: {}", name);`

**src/lib.rs**
- Ligne 26: `println!("Available audio output devices:");`
- Ligne 27: `println!("(Devices marked with ✓ support 44.1kHz stereo output)\n");`
- Ligne 63: `println!("  {} {}{}", validation_mark, name, default_mark);`
- Ligne 79: `println!("      Supported rates: {}", rates.join(", "));`
- Ligne 87: `println!("  No audio output devices found");`
- Ligne 91: `eprintln!("Error listing audio devices: {}", e);`
- Ligne 96: `println!("\nDevice selection will automatically try multiple strategies:");`
- Ligne 97: `println!("  1. Specified device (--output-device)");`
- Ligne 98: `println!("  2. System default device");`
- Ligne 99: `println!("  3. First available device");`
- Ligne 100: `println!("  4. Platform-specific fallbacks");`
- Ligne 103: `println!("\nLinux-specific devices that will be tried:");`
- Ligne 104: `println!("  - pulse (PulseAudio)");`
- Ligne 105: `println!("  - default (ALSA default)");`
- Ligne 106: `println!("  - pipewire (PipeWire)");`
- Ligne 107: `println!("  - hw:0,0 (Hardware device)");`
- Ligne 110: `println!();`

**src/server.rs**
- Ligne 10: `macro_rules! rt_eprintln {`
- Ligne 15: `macro_rules! rt_eprintln {`
- Ligne 17: `eprintln!($($arg)*);`
- Ligne 72: `println!("OSC server listening on {}", addr);`
- Ligne 100: `rt_eprintln!("[OSC WARNING] Command queue full - dropping message");`
- Ligne 106: `rt_eprintln!("[OSC ERROR] Failed to receive: {}", err);`
- Ligne 162: `self.print_samples();`
- Ligne 219: `println!(`
- Ligne 226: `println!("Error: Invalid source parameter type");`
- Ligne 230: `println!("Error: No source specified in /play message");`
- Ligne 308: `self.print_samples();`
- Ligne 312: `println!("Unknown OSC address: {}", msg.addr);`
- Ligne 506: `fn print_samples(&self) {`
- Ligne 507: `println!("=== Sample Library Status ===");`
- Ligne 512: `println!("No sample directories found");`
- Ligne 516: `println!("Found {} sample directories:", folders.len());`
- Ligne 519: `println!(`
- Ligne 526: `println!("      (no .wav files)");`
- Ligne 529: `println!("      [{}] {}", sample_idx, file_name);`
- Ligne 534: `println!("=== Usage Examples ===");`
- Ligne 535: `println!("  /play s sample sample_name kick sample_number 0");`
- Ligne 536: `println!("  /play s sample sample_name bass sample_number 2.5");`
- Ligne 537: `println!("==============================");`

**build.rs**
- Ligne 61: `println!(`
- Ligne 96: `println!("cargo:rerun-if-changed=static/tables");`
- Ligne 97: `println!("cargo:rerun-if-changed=build.rs");`

**src/main.rs**
- Ligne 94: `fn print_banner(`
- Ligne 101: `println!("\n");`
- Ligne 102: `println!(" ▗▄▄▖▄▄▄  ▗▖▗▞▀▜▌    Sample rate: {}", sample_rate);`
- Ligne 103: `println!("▐▌  █   █ ▐▌▝▚▄▟▌    Buffer size: {}", buffer_size);`
- Ligne 104: `println!(`
- Ligne 108: `println!("▝▚▄▄▖     ▐▙▄▞▘      OSC server: {}:{}", osc_host, osc_port);`
- Ligne 109: `println!("\n");`
- Ligne 124: `print_banner(`
- Ligne 150: `println!(`
- Ligne 158: `print!("Engine config: {} voices", args.max_voices);`
- Ligne 160: `print!(" | Output: {}", device);`
- Ligne 162: `println!();`
- Ligne 175: `println!("Starting audio engine...");`
- Ligne 203: `eprintln!("Failed to create OSC server: {}", e);`
- Ligne 225: `println!("Ready ✓");`
- Ligne 229: `Ok(_) => println!("Audio thread exited"),`
- Ligne 230: `Err(_) => eprintln!("Audio thread panicked"),`
- Ligne 238: `Ok(_) => println!("OSC thread exited"),`
- Ligne 239: `Err(_) => eprintln!("OSC thread panicked"),`

**src/memory/samplib.rs**
- Ligne 144: `eprintln!(`
- Ligne 375: `println!(`
- Ligne 387: `println!(`
- Ligne 393: `println!("Successfully pre-loaded {} total samples", total_loaded);`

**src/memory/predictive.rs**
- Ligne 385: `eprintln!("Sample loader queue full - dropping request");`
- Ligne 428: `eprintln!("Failed to send loaded sample message - audio thread may be busy");`
- Ligne 436: `eprintln!(`

## Résumé

**Total des utilisations dans core/**: 193 occurrences de print/println/eprintln
**Total des utilisations dans engine/**: 71 occurrences de print/println/eprintln

**Total général**: 264 occurrences de print/println/eprintln dans les deux dossiers combinés.

Les utilisations se répartissent principalement entre :
- Messages de debug et d'information (println!)
- Messages d'erreur (eprintln!)
- Fonctions de debug spécialisées (debug_print)
- Banners et messages de démarrage
- Logs de statut et d'état des composants