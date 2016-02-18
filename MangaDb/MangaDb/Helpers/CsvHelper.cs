﻿using CsvHelper;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

using MangaDb.Extensions;

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

        public void SaveRecords<T>(string fileName, IEnumerable<T> records)
        {
            using (var writer = new CsvWriter(new StreamWriter(File.Open(fileName, FileMode.OpenOrCreate))))
            {
                writer.WriteRecords(records);
            }
        }

        public void AppendRecords<T>(string fileName, IEnumerable<T> records)
        {
            using (var writer = new CsvWriter(new StreamWriter(File.Open(fileName, FileMode.Append))))
            {
                records.ForEach(r => writer.WriteRecord(r));
            }
        }
    }
}
