use stubs::DataPacket

pub trait DataSturcture {
    pub fn update(&mut self, dp : DataPacket) -> Result<(), Error>;
}