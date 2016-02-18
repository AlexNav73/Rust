using MangaDb.Configurations;
using MangaDb.Helpers;
using System.Threading.Tasks;
using MangaDb.Results;
using MangaDb.Contexts;

namespace MangaDb.Modules.Implementations
{
    public class Downloader : IModule
    {
        public async Task<object> Execute(object context)
        {
            var conf = (ConveyorContext)context;
            HttpHelper helper = new HttpHelper();

            return new DownloaderResult()
            {
                Page = await helper.DownloadPage(conf.Config.ListSiteUrl),
                Context = conf
            };
        }

    }
}
