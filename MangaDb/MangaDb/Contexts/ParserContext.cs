using MangaDb.Configurations;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace MangaDb.Contexts
{
    public class ParserContext
    {
        private string _tld = null;
        public string Tld
        {
            get { return _tld; }
            set
            {
                Regex r = new Regex(@"(http.?:\/\/[\w.]*)");
                var m = r.Match(value);
                _tld = m.Groups[0].Value;
            }
        }

        public string Page { get; set; }
        public MainConfig Config { get; set; }
    }
}
