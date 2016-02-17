using MangaDb.Configurations;
using MangaDb.Contexts;
using MangaDb.Helpers;
using MangaDb.Modules;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MangaDb.Modules
{
    public class Downloader : IModule
    {
        public async Task<object> Execute(object context)
        {
            //var conf = context as MainConfig;
            //if (conf == null) return null;

            HttpHelper helper = new HttpHelper();

            return new ParserContext()
            {
                //Page = helper.DownloadPage(conf.ListLocationUrl),
                Page = await helper.DownloadPage("http://grouple.ru/user/652147/bookmarks"),
                //Config = conf
                Config = null
            };
        }

    }
}
