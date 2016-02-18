using MangaDb.Attributes;
using MangaDb.Entities;
using MangaDb.Extensions;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;
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

            items.ForEach(e => ProcessFields(writer, e));

            writer.RenderEndTag();
        }

        private void ProcessFields(HtmlTextWriter writer, ListEntry entry)
        {
            writer.RenderBeginTag(HtmlTextWriterTag.Tr);

            ProcessCustomFields(writer, GetCustomData(entry));

            GetData(entry).ForEach(el =>
            {
                writer.RenderBeginTag(HtmlTextWriterTag.Td);
                writer.Write(el.ToString());
                writer.RenderEndTag();
            });

            writer.RenderEndTag();
        }

        private void RenderLink(HtmlTextWriter writer, string link)
        {
            writer.AddAttribute(HtmlTextWriterAttribute.Href, link);
            writer.RenderBeginTag(HtmlTextWriterTag.A);
            writer.Write("Click     ");
            writer.RenderEndTag();
        }

        private void RenderImage(HtmlTextWriter writer, string link)
        {
            writer.AddAttribute(HtmlTextWriterAttribute.Src, link);
            writer.AddAttribute(HtmlTextWriterAttribute.Width, "42");
            writer.AddAttribute(HtmlTextWriterAttribute.Height, "42");
            writer.RenderBeginTag(HtmlTextWriterTag.Img);
            writer.RenderEndTag();
        }

        private void ProcessCustomFields(HtmlTextWriter writer, IEnumerable<object> data)
        {
            object[] items = data.ToArray();

            writer.RenderBeginTag(HtmlTextWriterTag.Td);
            RenderLink(writer, (string)items[0]);
            writer.RenderEndTag();

            writer.RenderBeginTag(HtmlTextWriterTag.Td);
            RenderImage(writer, (string)items[1]);
            writer.RenderEndTag();
        }

        private IEnumerable<string> GetHeaders(IEnumerable<ListEntry> items)
        {
            return typeof(ListEntry).GetProperties()
                .Where(p => p.GetCustomAttribute<ExcludeAttribute>() == null)
                .Select(p => p.Name);
        }

        private IEnumerable<object> GetData(ListEntry item)
        {
            return typeof(ListEntry).GetProperties()
                .Where(p => p.GetCustomAttribute<IncludeAttribute>() != null)
                .Select(p => p.GetValue(item));
        }

        private IEnumerable<object> GetCustomData(ListEntry item)
        {
            return typeof(ListEntry).GetProperties()
                .Where(p => 
                    p.GetCustomAttribute<IncludeAttribute>() == null &&
                    p.GetCustomAttribute<ExcludeAttribute>() == null)
                .Select(p => p.GetValue(item));
        }
    }
}
