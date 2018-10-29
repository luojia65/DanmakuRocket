using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Drawing.Drawing2D;
using System.Linq;
using System.Text;
using System.Windows.Forms;

namespace DanmuDisplayerWin
{
    public partial class DanmakuForm : Form
    {
        private string Msg;

        public DanmakuForm(string Msg)
        {
            this.Msg = Msg;
            InitializeComponent();
        }

        private void DanmakuForm_Load(object sender, EventArgs e)
        {
            this.Paint += Form1_Paint;
            Left = SystemInformation.PrimaryMonitorSize.Width;
            TransparencyKey = Color.FromArgb(233, 233, 233);
        }


        private void Ticker_Tick(object sender, EventArgs e)
        {
            if (Left > -2000)
            {
                Left = Location.X - 3;
            }
            else
            {
                Dispose();
            }
        }

        void Form1_Paint(object sender, PaintEventArgs e)
        {
            //Graphics g = e.Graphics;
            //string s = "Outline";
            //RectangleF rect = this.ClientRectangle;
            //Font font = this.Font;
            //StringFormat format = StringFormat.GenericTypographic;
            //float dpi = g.DpiY;
            //using (GraphicsPath path= GetStringPath(s, dpi, rect, font, format))
            //{
            //    g.DrawPath(Pens.Black, path);
            //}


            Graphics g = e.Graphics;
            string s = Msg;
            Font font = Font;
            var ms = g.MeasureString(s, font);
            RectangleF rect = new RectangleF(0, 0, 2000, 300);//new RectangleF(0, 0, ms.Width, ms.Height);
            Console.WriteLine(rect);
            StringFormat format = StringFormat.GenericTypographic;
            float dpi = g.DpiY;
            float sh = 2f;
            using (GraphicsPath path = GetStringPath(s, dpi, rect, font, format))
            {
                //阴影代码
                RectangleF off = rect;

                off.Offset(sh, sh);//阴影偏移
                using (GraphicsPath offPath = GetStringPath(s, dpi, off, font, format))
                {
                    Brush b = new SolidBrush(Color.Black);
                    g.FillPath(b, offPath);
                    b.Dispose();
                }
                off.Offset(-sh, 0);//阴影偏移
                using (GraphicsPath offPath = GetStringPath(s, dpi, off, font, format))
                {
                    Brush b = new SolidBrush(Color.Black);
                    g.FillPath(b, offPath);
                    b.Dispose();
                }
                off.Offset(-sh, 0);//阴影偏移
                using (GraphicsPath offPath = GetStringPath(s, dpi, off, font, format))
                {
                    Brush b = new SolidBrush(Color.Black);
                    g.FillPath(b, offPath);
                    b.Dispose();
                }
                off.Offset(0, -sh);//阴影偏移
                using (GraphicsPath offPath = GetStringPath(s, dpi, off, font, format))
                {
                    Brush b = new SolidBrush(Color.Black);
                    g.FillPath(b, offPath);
                    b.Dispose();
                }
                off.Offset(0, -sh);//阴影偏移
                using (GraphicsPath offPath = GetStringPath(s, dpi, off, font, format))
                {
                    Brush b = new SolidBrush(Color.Black);
                    g.FillPath(b, offPath);
                    b.Dispose();
                }
                off.Offset(sh, 0);//阴影偏移
                using (GraphicsPath offPath = GetStringPath(s, dpi, off, font, format))
                {
                    Brush b = new SolidBrush(Color.Black);
                    g.FillPath(b, offPath);
                    b.Dispose();
                }
                off.Offset(sh, 0);//阴影偏移
                using (GraphicsPath offPath = GetStringPath(s, dpi, off, font, format))
                {
                    Brush b = new SolidBrush(Color.Black);
                    g.FillPath(b, offPath);
                    b.Dispose();
                }
                off.Offset(0, sh);//阴影偏移
                using (GraphicsPath offPath = GetStringPath(s, dpi, off, font, format))
                {
                    Brush b = new SolidBrush(Color.Black);
                    g.FillPath(b, offPath);
                    b.Dispose();
                }
                //g.SmoothingMode = SmoothingMode.AntiAlias;//设置字体质量
                //Pen p = new Pen(Brushes.Black);
                //p.Width = 2.0f;
                //g.DrawPath(p, path);//绘制轮廓（描边）
                g.FillPath(Brushes.White, path);//填充轮廓（填充）
            }

        }

        GraphicsPath GetStringPath(string s, float dpi, RectangleF rect, Font font, StringFormat format)
        {
            GraphicsPath path = new GraphicsPath();
            // Convert font size into appropriate coordinates
            float emSize = dpi * font.SizeInPoints / 72;
            path.AddString(s, font.FontFamily, (int)font.Style, emSize, rect, format);

            return path;
        }

        //private void trackBar1_Scroll(object sender, EventArgs e)
        //{
        //    Console.WriteLine(trackBar1.Value);
        //    this.Refresh();
        //}
    }
}
