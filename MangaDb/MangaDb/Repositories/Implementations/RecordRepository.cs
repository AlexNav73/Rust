using MangaDb.Configurations;
using MangaDb.Entities;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MangaDb.Repositories.Implementations
{
    public class RecordRepository : IRepository<ListEntry>
    {
        private Helpers.CsvHelper _helper = new Helpers.CsvHelper();
        private string _dbFileName = null;

        public void Init(MainConfig conf)
        {
            _dbFileName = conf.FilePath;
        }

        public void AddNew(ListEntry item)
        {
            _helper.AppendRecords(_dbFileName, new[] { item });
        }

        public void AddRange(IEnumerable<ListEntry> items)
        {
            _helper.AppendRecords(_dbFileName, items);
        }

        public List<ListEntry> GetAll()
        {
            return _helper.GetRecords<ListEntry>(_dbFileName);
        }

        public void Save() { }

        public void SaveAll(IEnumerable<ListEntry> items)
        {
            _helper.SaveRecords<ListEntry>(_dbFileName, items);
        }
    }
}
