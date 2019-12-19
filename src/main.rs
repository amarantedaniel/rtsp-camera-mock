use gstreamer_rtsp_server::prelude::*;

use {
  glib::{MainContext, MainLoop},
  gstreamer_rtsp_server::{RTSPMediaFactory, RTSPServer},
};

fn main() {
  gstreamer::init().unwrap();

  let media_factory = RTSPMediaFactory::new();
  media_factory.set_launch("videotestsrc ! x264enc ! rtph264pay name=pay0 pt=96");
  media_factory.set_shared(true);

  let main_context = MainContext::default();

  let server = RTSPServer::new();
  let source_id = server.attach(Some(&main_context));

  let end_point = "/stream";

  let mount_points = server.get_mount_points().unwrap();
  mount_points.add_factory(end_point, &media_factory);

  let main_loop = MainLoop::new(Some(&main_context), false);

  let address = server.get_address().unwrap();
  let port = server.get_bound_port();
  let location = format!("rtsp://{}:{}{}", address, port, end_point);

  println!("Server available at {}", location);

  main_loop.run();

  glib::source_remove(source_id);
}
