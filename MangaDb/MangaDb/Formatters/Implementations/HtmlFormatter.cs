using MangaDb.Entities;
using MangaDb.Extensions;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Web.UI;

namespace MangaDb.Formatters.Implementations
{
    public class HtmlFormatter : IFormatter<ListEntry>
    {
        public string Format(IEnumerable<ListEntry> items)
        {
            StringWriter strWriter = new StringWriter();

            using (var writer = new HtmlTextWriter(strWriter))
            {
                CreateHeader(writer, items);
            }

            return strWriter.ToString();
        }

        private void CreateHeader(HtmlTextWriter writer, IEnumerable<ListEntry> items)
        {
            writer.RenderBeginTag(HtmlTextWriterTag.Html);

            CreateBody(writer, items);

            writer.RenderEndTag();
        }

        private void CreateBody(HtmlTextWriter writer, IEnumerable<ListEntry> items)
        {
            writer.RenderBeginTag(HtmlTextWriterTag.Table);

            GetHeaders(items).ForEach(e =>
            {
                writer.RenderBeginTag(HtmlTextWriterTag.Th);
                writer.Write(e);
                writer.RenderEndTag();
            });

            items.ForEach(e =>
            {
                writer.RenderBeginTag(HtmlTextWriterTag.Tr);

                GetData(e).ForEach(el =>
                {
                    writer.RenderBeginTag(HtmlTextWriterTag.Td);
                    writer.Write(el.ToString());
                    writer.RenderEndTag();
                });
                writer.RenderEndTag();
            });

            writer.RenderEndTag();
        }

        public void RenderLink(HtmlTextWriter writer, IEnumerable<object> data)
        {
            writer.AddAttribute(HtmlTextWriterAttribute.Href, (string)data.First());
            writer.RenderBeginTag(HtmlTextWriterTag.A);
            writer.Write("Click");
            writer.RenderEndTag();
        }

        private IEnumerable<string> GetHeaders(IEnumerable<ListEntry> items)
        {
            return typeof(ListEntry).GetProperties().Select(p => p.Name);
        }

        private IEnumerable<object> GetData(ListEntry item)
        {
            return typeof(ListEntry).GetProperties().Select(p => p.GetValue(item));
        }
    }
}
