using CsvHelper;
using MangaDb.Entities;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MangaDb.Helpers
{
    public class CsvHelper
    {
        public List<T> GetRecords<T>(string fileName)
        {
            using (var reader = new CsvReader(new StreamReader(File.Open(fileName, FileMode.OpenOrCreate))))
            {
                return reader.GetRecords<T>().ToList();
            }
        }

        public void SaveRecords<T>(string fileName, List<T> records)
        {
            using (var writer = new CsvWriter(new StreamWriter(File.Open(fileName, FileMode.OpenOrCreate))))
            {
                writer.WriteRecords(records);
            }
        }
    }
}
