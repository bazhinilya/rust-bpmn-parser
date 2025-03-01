use rust_xlsxwriter::workbook::Workbook;

pub struct Excel {
    workbook: Workbook,
}

impl Excel {
    pub fn new() -> Self {
        let workbook = Workbook::new();
        Excel { workbook }
    }

    pub fn write_to_excel(
        &mut self,
        data: Vec<(String, String)>,
        name: &str,
        date_time: &str,
    ) -> &mut Self {
        let worksheet = self.workbook.add_worksheet();
        let _ = worksheet.set_name(name);

        let mut row = 0;
        for line in &data {
            let _ = worksheet.write(row, 0, line.0.to_owned());
            let _ = worksheet.write(row, 1, line.1.to_owned());
            row += 1;
        }
        let _ = self.workbook.save(format!("output/{}.xlsx", date_time));
        self
    }

    pub fn write_to_excel_single(
        &mut self,
        data: Vec<String>,
        name: &str,
        date_time: &str,
    ) -> &mut Self {
        let worksheet = self.workbook.add_worksheet();
        let _ = worksheet.set_name(name);

        let mut row = 0;
        for line in &data {
            let _ = worksheet.write(row, 0, line.to_owned());
            row += 1;
        }
        let _ = self.workbook.save(format!("output/{}.xlsx", date_time));
        self
    }
}
