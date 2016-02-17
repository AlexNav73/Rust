using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MangaDb.Entities;

namespace MangaDb.Results
{
    public class ParseResult
    {
        public DownloaderResult DownloaderResult { get; set; }
        public List<ListEntry> Entries { get; set; }
    }
}
