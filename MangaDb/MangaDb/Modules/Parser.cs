using MangaDb.Contexts;
using MangaDb.Entities;
using MangaDb.Processors;
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

            string pattern = @"<a href='.*' class='site-element site_2' .*</a>";
            //Regex reg = new Regex(cont.Config.ListRegex, RegexOptions.Compiled);
            Regex reg = new Regex(pattern, RegexOptions.Compiled);
            List<ListEntry> result = new List<ListEntry>();
            foreach (Match match in reg.Matches(cont.Page))
            {
                ProcessMatch(match, result);
            }

            return null;
        }

        private void ProcessMatch(Match match, List<ListEntry> result)
        {
            Regex r = new Regex(@"<a .*>(.*)<sup>.*");
            foreach (Match ma in r.Matches(match.Value))
            {
                result.Add(new ListEntry()
                {
                    Name = ma.Groups[1].Value,
                });
            }
        }
    }
}
