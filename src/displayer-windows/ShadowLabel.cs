using System;
using System.Collections.Generic;
using System.Drawing;
using System.Drawing.Drawing2D;
using System.Linq;
using System.Text;
using System.Windows.Forms;

namespace DanmuDisplayerWin
{
    class ShadowLabel: Label
    {
        //获取需要绘制的path 
        GraphicsPath GetStringPath(string s, float dpi, RectangleF rect, Font font, StringFormat format)
        {
            GraphicsPath path = new GraphicsPath(); //计算文字高度 
            float emSize = dpi * font.SizeInPoints / 72; //向path中添加字符串及相应信息 
            path.AddString(s, font.FontFamily, (int)font.Style, emSize, rect, format);
            return path;
        } 

        //重写label控件的paint方法 
        protected override void OnPaint(PaintEventArgs e)
        {
            base.OnPaint(e);
            if (Text == null || Text.Length < 1) return;
            Graphics g = e.Graphics;//相当于画笔 
            RectangleF rect = ClientRectangle;//获取控件的工作区  
                                              //计算垂直偏移量
            float dy = (Height - g.MeasureString(Text, Font).Height) / 2.0f;
            //计算水平偏移 
            float dx = (Width - g.MeasureString(Text, Font).Width) / 2.0f; 
            //将文字显示的工作区偏移dx,dy，实现文字居中、水平居中、垂直居中 
            rect.Offset(dx, dy);
            StringFormat format = StringFormat.GenericTypographic;
            float dpi = g.DpiY;
            using (GraphicsPath path = GetStringPath(Text, dpi, rect, Font, format))
            {

                RectangleF off = rect; off.Offset(5, 5);//阴影偏移 
                using (GraphicsPath offPath = GetStringPath(Text, dpi, off, Font, format)) {
                    Brush b = new SolidBrush(Color.FromArgb(100, 0, 0, 0));//阴影颜色 
                    g.FillPath(b, offPath); //  
                    g.DrawPath(Pens.AliceBlue, offPath);//给阴影描边 
                    b.Dispose();
                }

                g.SmoothingMode = SmoothingMode.AntiAlias;//设置字体质量 
                g.FillPath(Brushes.Orange, path);//填充轮廓（填充） fillBrush 填充色 
                g.DrawPath(Pens.Red, path);//绘制轮廓（描边） borderPen 描边色 
            }
        }
    }
}
