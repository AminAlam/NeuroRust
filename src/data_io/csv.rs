// A module to read, write and manipulate csv files

// Written by Amin Alam in 2024

use std::fs::File;
use std::io::{BufReader, BufWriter};
use csv::{Reader, Writer, StringRecord};

/// A class to read, write and manipulate csv files
/// 
/// # Arguments
/// 
/// * `file_path` - A string slice that holds the path to the csv file
/// * `reader` - A csv::Reader object that reads the csv file
/// * `writer` - A csv::Writer object that writes to the csv file
/// * `headers` - A csv::StringRecord object that holds the headers of the csv file
/// * `is_open` - A boolean that indicates if the file is open
/// 
/// # Examples
/// 
/// ```
/// let csv_io = CsvIO::new("data.csv");
/// ```
/// 
/// # Note
/// 
/// This class is not thread safe and should not be shared between threads
pub struct CsvIO {
    file_path: String,
    reader: Reader<BufReader<File>>,
    writer: Writer<BufWriter<File>>,
    headers: StringRecord,
    is_open: bool,
}


/// Implementation of the CsvIO class
/// 
/// # Methods
/// 
/// * `new` - Creates a new CsvIO object
/// 
/// # Examples
/// 
/// ```
/// let csv_io = CsvIO::new("data.csv");
/// ```
impl CsvIO {
    /// Creates a new CsvIO object
    /// 
    /// # Arguments
    /// 
    /// * `file_path` - A string slice that holds the path to the csv file
    /// 
    /// # Examples
    /// 
    /// ```
    /// let csv_io = CsvIO::new("data.csv");
    /// ```
    /// 
    fn new(file_path: &str) -> Self {
        let reader_file = File::open(file_path).expect("File not found");
        let mut reader = Reader::from_reader(BufReader::new(reader_file));
        let writer_file = File::create(file_path).expect("Could not create file");
        let writer = Writer::from_writer(BufWriter::new(writer_file));

        let headers: StringRecord = reader.headers().expect("No headers found").clone();

        Self {
            reader: reader,
            writer: writer,
            file_path: file_path.to_string(),
            headers: headers,
            is_open: true,
        }
    }

    /// Reads the next record from the csv file
    /// 
    /// # Arguments
    /// 
    /// * `self` - A mutable reference to the CsvIO object
    /// 
    /// # Returns
    /// 
    /// A csv::StringRecord object that holds the record
    /// 
    /// # Examples
    /// 
    /// ```
    /// let record = csv_io.read_record();
    /// ```
    /// 
    fn read_record(&mut self) -> StringRecord {
        self.reader.records().next().expect("No records found").expect("Error reading record")
    }

    /// Reads all records from the csv file
    /// 
    /// # Arguments
    /// 
    /// * `self` - A mutable reference to the CsvIO object
    /// 
    /// # Returns
    /// 
    /// A vector of csv::StringRecord objects that holds the records
    /// 
    /// # Examples
    /// 
    /// ```
    /// let records = csv_io.read_records();
    /// ```
    /// 
    /// 
    fn read_records(&mut self) -> Vec<StringRecord> {
        let mut records: Vec<StringRecord> = Vec::new();
        for record in self.reader.records() {
            records.push(record.expect("Error reading record"));
        }
        records
    }

    /// Writes a record to the csv file
    /// 
    /// # Arguments
    /// 
    /// * `self` - A mutable reference to the CsvIO object
    /// * `record` - A csv::StringRecord object that holds the record
    /// 
    /// # Examples
    /// 
    /// ```
    /// csv_io.write_record(record);
    /// ```
    /// 
    /// # Note
    /// 
    /// This method does not write the record to the file immediately.
    /// Use the `flush` method to write the record to the file.
    /// 
    /// # See
    /// 
    /// * `flush` - Writes the record to the file
    /// 
    fn write_record(&mut self, record: StringRecord) {
        self.writer.write_record(&record).expect("Error writing record");
    }

    /// Writes a group of records to the csv file
    /// 
    /// # Arguments
    /// 
    /// * `self` - A mutable reference to the CsvIO object
    /// * `records` - A vector of csv::StringRecord objects that holds the records
    /// 
    /// # Examples
    /// 
    /// ```
    /// csv_io.write_records(records);
    /// ```
    /// 
    /// # Note
    /// 
    /// This method does not write the records to the file immediately.
    /// Use the `flush` method to write the records to the file.
    /// 
    /// # See
    /// 
    /// * `flush` - Writes the records to the file
    ///
    fn write_records(&mut self, records: Vec<StringRecord>) {
        for record in records {
            self.write_record(record);
        }
    }

    /// saves all the changes to the file
    /// 
    /// # Arguments
    /// 
    /// * `self` - A mutable reference to the CsvIO object
    /// 
    /// # Examples
    /// 
    /// ```
    /// csv_io.save();
    /// ```
    /// 
    /// # Note
    /// 
    /// This method writes all the changes to the actual file on disk
    /// 
    fn save(&mut self) {
        self.writer.flush().expect("Error saving file");
    }

    /// Closes the file
    /// 
    /// # Arguments
    /// 
    /// * `self` - A mutable reference to the CsvIO object
    /// 
    /// # Examples
    /// 
    /// ```
    /// csv_io.close();
    /// ```
    /// 
    /// # Note
    /// 
    /// This method closes the file and frees up resources
    /// 
    fn close(&mut self) {
        self.is_open = false;
    }
} 