using MangaDb.Configurations;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace MangaDb.Results
{
    public class DownloaderResult
    {
        public string Page { get; set; }
        public MainConfig Config { get; set; }
    }
}
