using MangaDb.Configurations;
using MangaDb.Helpers;
using System.Threading.Tasks;
using MangaDb.Results;

namespace MangaDb.Modules
{
    public class Downloader : IModule
    {
        public async Task<object> Execute(object context)
        {
            var conf = context as MainConfig;
            if (conf == null) return null;

            HttpHelper helper = new HttpHelper();

            return new DownloaderResult()
            {
                Page = await helper.DownloadPage(conf.ListSiteUrl),
                Config = conf
            };
        }

    }
}
