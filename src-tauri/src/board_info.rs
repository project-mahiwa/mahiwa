use std::collections::HashMap;
pub struct Board {
    //ボードの情報
    board: String,
    pio_envrionment_name: String,
    soc: String,
    api: String,
    ram: i32,
    rom: i32,
    core: i8,
    clock: i32,
    //機能判定
    gpio: bool,
    display: bool,
    wifi: bool,
    bluetooth: bool,
    ethernet: bool,
    usb: bool,
    //pin位置
    //uart
    uart_tx: i32,
    uart_rx: i32,
    //i2c
    i2c_sda: i32,
    i2c_scl: i32,
    //spi
    spi_mosi: i32,
    spi_miso: i32,
    //path(SvelteKit側で使うためのやつ)
}
impl Board {
    pub fn get_pio_envrionment_name(&self) -> &String {
        &self.pio_envrionment_name
    }
}
pub fn get_boards() -> HashMap<String, Board> {
    let mut boards = HashMap::new();
    boards.insert(
        "M5 Stack Core2".to_string(),
        Board {
            board: "M5 Stack Core2".to_string(),
            pio_envrionment_name: "m5stack-core2".to_string(),
            soc: "ESP32-D0WDQ6-V3".to_string(),
            api: "ESP32".to_string(),
            ram: 8,
            rom: 16,
            core: 2,
            clock: 240,
            gpio: true,
            display: true,
            wifi: true,
            bluetooth: true,
            ethernet: false,
            usb: true,
            uart_tx: 1,
            uart_rx: 3,
            i2c_sda: 21,
            i2c_scl: 22,
            spi_mosi: 23,
            spi_miso: 19,
        },
    );
    //raspberry pi pico w
    boards.insert(
        "Raspberry Pi Pico W".to_string(),
        Board {
            board: "Raspberry Pi Pico W".to_string(),
            // これどうなの↓
            pio_envrionment_name: "pico".to_string(),
            soc: "RP2040".to_string(),
            api: "RP2040".to_string(),
            ram: 264,
            rom: 2,
            core: 1,
            clock: 133,
            gpio: true,
            display: false,
            wifi: false,
            bluetooth: false,
            ethernet: false,
            usb: true,
            uart_tx: 1,
            uart_rx: 3,
            i2c_sda: 21,
            i2c_scl: 22,
            spi_mosi: 23,
            spi_miso: 19,
        },
    );

    //Seeed Studio XIAO ESP32C3
    boards.insert(
        "Seeed Studio XIAO ESP32C3".to_string(),
        Board {
            board: "Seeed Studio XIAO ESP32C3".to_string(),
            pio_envrionment_name: "seeed_xiao_esp32c3".to_string(),
            soc: "ESP32-C3".to_string(),
            api: "ESP32".to_string(),
            ram: 400,
            rom: 4,
            core: 1,
            clock: 160,
            gpio: true,
            display: false,
            wifi: true,
            bluetooth: true,
            ethernet: false,
            usb: true,
            uart_tx: 1,
            uart_rx: 3,
            i2c_sda: 21,
            i2c_scl: 22,
            spi_mosi: 23,
            spi_miso: 19,
        },
    );

    return boards;
}
