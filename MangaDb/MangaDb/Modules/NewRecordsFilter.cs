using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MangaDb.Entities;
using MangaDb.Results;

namespace MangaDb.Modules
{
    public class NewRecordsFilter : IModule
    {
        public async Task<object> Execute(object context)
        {
            var res = (ParseResult) context;

            var path = res.DownloaderResult.Config.FilePath;
            var helper = new Helpers.CsvHelper();

            if (File.Exists(path))
            {
                var records = helper.GetRecords<ListEntry>(path);
                var newRecords = res.Entries.Where(r => !records.Contains(r)).ToList();
                helper.AppendRecords(path, newRecords);
                return newRecords;
            }
            helper.SaveRecords(path, res.Entries);

            return res.Entries;
        }
    }
}
