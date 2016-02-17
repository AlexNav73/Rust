using MangaDb.Contexts;
using MangaDb.Entities;
using MangaDb.Modules;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace MangaDb.Modules
{
    public class Parser : IModule
    {
        public async Task<object> Execute(object context)
        {
            var cont = (ParserContext)context;

            string pattern = @"<a href='.*' class='site-element .*' .*</a>";
            string p2 = @"<a href='(.*)'.* title='(.*: (.*))?'.*>(.*)<sup>.*>(.*)<\/a>.*rel='(.*)' .*";

            //Regex reg = new Regex(cont.Config.ListRegex, RegexOptions.Compiled);
            Regex reg = new Regex(pattern, RegexOptions.Compiled);
            List<ListEntry> result = new List<ListEntry>();
            foreach (Match match in reg.Matches(cont.Page))
            {
                ParseRecord(p2, cont.Tld, match, result);
            }

            return result;
        }

        private void ParseRecord(string pattern, string tld, Match record, List<ListEntry> result)
        {
            foreach (Match attibutes in (new Regex(pattern)).Matches(record.Value))
            {
                List<string> vals = new List<string>(attibutes.Groups.Count - 1);
                foreach (Group g in attibutes.Groups)
                    vals.Add(g.Value);
                var entry = new ListEntry().Init(vals);
                entry.Url = tld + entry.Url;
                result.Add(entry);
            }
        }
    }
}
