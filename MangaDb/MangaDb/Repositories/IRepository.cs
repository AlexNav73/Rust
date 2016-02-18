using MangaDb.Configurations;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MangaDb.Repositories
{
    public interface IRepository<T>
    {
        void Init(MainConfig conf);
        List<T> GetAll();
        void SaveAll(IEnumerable<T> items);
        void Save();
        void AddNew(T item);
        void AddRange(IEnumerable<T> items);
    }
}
