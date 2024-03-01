use anyhow::Result;

use super::SchemaClassFieldData;

use crate::os::{Address, Process};

pub struct SchemaClassInfo<'a> {
    process: &'a Process,
    address: Address,
    class_name: String,
}

impl<'a> SchemaClassInfo<'a> {
    pub fn new(process: &'a Process, address: Address, class_name: &str) -> Self {
        Self {
            process,
            address,
            class_name: class_name.to_string(),
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.class_name
    }

    pub fn fields(&self) -> Result<Vec<SchemaClassFieldData>> {
        let address = self.process.read_memory::<usize>(self.address + 0x28)?;

        if address == 0 {
            return Ok(Vec::new());
        }

        let count = self.fields_count()?;

        let fields: Vec<SchemaClassFieldData> = (address..address + count as usize * 0x20)
            .step_by(0x20)
            .map(|address| SchemaClassFieldData::new(self.process, address.into()))
            .collect();

        Ok(fields)
    }

    pub fn fields_count(&self) -> Result<u16> {
        self.process.read_memory::<u16>(self.address + 0x1C)
    }

    pub fn parent(&self) -> Result<Option<SchemaClassInfo>> {
        let address = Address::from(self.process.read_memory::<usize>(self.address + 0x38)?);

        if address.is_zero() {
            return Ok(None);
        }

        let parent = Address::from(self.process.read_memory::<usize>(address + 0x8)?);

        let name_ptr = self.process.read_memory::<usize>(parent + 0x8)?;
        let name = self.process.read_string(name_ptr.into())?;

        Ok(Some(SchemaClassInfo::new(
            self.process,
            parent.into(),
            &name,
        )))
    }
}
