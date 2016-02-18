using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MangaDb.Entities;
using MangaDb.Results;
using MangaDb.Repositories;

namespace MangaDb.Modules.Implementations
{
    public class UpdateDb : IModule
    {
        private IRepository<ListEntry> _repository;

        public UpdateDb(IRepository<ListEntry> repo)
        {
            _repository = repo;
        }

        public async Task<object> Execute(object context)
        {
            var res = (ParseResult) context;
            _repository.Init(res.DownloaderResult.Context.Config);

            var path = res.DownloaderResult.Context.Config.FilePath;

            if (File.Exists(path))
            {
                var records = _repository.GetAll();
                var newRecords = Enumerable.Except(res.Entries, records).ToList();
                _repository.AddRange(newRecords);
                return res.Entries;
            }
            _repository.SaveAll(res.Entries);

            return res.Entries;
        }
    }
}
