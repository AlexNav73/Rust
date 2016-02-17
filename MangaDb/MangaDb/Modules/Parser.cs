using MangaDb.Entities;
using MangaDb.Modules;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Security.Policy;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using MangaDb.Configurations;
using MangaDb.Results;
using Group = System.Text.RegularExpressions.Group;

namespace MangaDb.Modules
{
    public class Parser : IModule
    {
        public async Task<object> Execute(object context)
        {
            var cont = (DownloaderResult)context;

            Regex reg = new Regex(cont.Config.RecordRegex, RegexOptions.Compiled);
            List<ListEntry> result = new List<ListEntry>();
            foreach (Match match in reg.Matches(cont.Page))
            {
                ParseRecord(cont.Config, match, result);
            }

            return new ParseResult()
            {
                DownloaderResult = cont,
                Entries = result
            };
        }

        private static void ParseRecord(MainConfig conf, Capture record, ICollection<ListEntry> result)
        {
            var reg = new Regex(conf.Entry.Regex);
            foreach (Match attibutes in reg.Matches(record.Value))
            {
                var vals = new List<string>(attibutes.Groups.Count - 1);
                vals.AddRange(from Group g in attibutes.Groups select g.Value);
                var entry = new ListEntry().Init(conf.Entry.Groups, vals);
                entry.Url = conf.GetLinkToPage(entry.Url);
                result.Add(entry);
            }
        }
    }
}
