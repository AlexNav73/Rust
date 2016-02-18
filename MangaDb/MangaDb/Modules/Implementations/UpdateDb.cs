using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MangaDb.Entities;
using MangaDb.Results;

namespace MangaDb.Modules.Implementations
{
    public class UpdateDb : IModule
    {
        public async Task<object> Execute(object context)
        {
            var res = (ParseResult) context;

            var path = res.DownloaderResult.Context.Config.FilePath;
            var repo = res.DownloaderResult.Context.Repository;

            if (File.Exists(path))
            {
                var records = repo.GetAll();
                var newRecords = res.Entries.Where(r => !records.Contains(r)).ToList();
                repo.AddRange(newRecords);
                return res.Entries;
            }
            repo.SaveAll(res.Entries);

            return res.Entries;
        }
    }
}
