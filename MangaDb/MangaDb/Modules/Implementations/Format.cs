using MangaDb.Entities;
using MangaDb.Formatters;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MangaDb.Modules.Implementations
{
    public class Format : IModule
    {
        private IFormatter<ListEntry> _formatter = null;

        public Format(IFormatter<ListEntry> formatter)
        {
            _formatter = formatter;
        }

        public async Task<object> Execute(object context)
        {
            var items = (List<ListEntry>)context;

            return _formatter.Format(items);
        }
    }
}
