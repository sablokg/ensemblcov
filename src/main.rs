mod annotate;
mod args;
mod autogenerate;
mod counts;
mod differential;
mod exon;
mod extract;
mod threaded;
use crate::annotate::generateannotations;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::autogenerate::generatecovid;
use crate::counts::convertcounts;
use crate::differential::differentialconvert;
use crate::exon::exonunwrap;
use crate::extract::geneunwrap;
use crate::threaded::threadedautogenerate;
use clap::Parser;
use figlet_rs::FIGfont;

/*
 Gaurav Sablok,
 gsablok@proton.me
*/

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("ensemblcov");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    let argparse = CommandParse::parse();
    match &argparse.command {
        Commands::ThreadedAuto { generate, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = threadedautogenerate(generate).unwrap();
                println!("The command has finished:{}", command);
            });
        }
        Commands::AutoGenerate { generate, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = generatecovid(generate).unwrap();
                println!(
                    "The gene conversion for the entire geneome has been written:{}",
                    command
                );
            });
        }
        Commands::GTFAnnotateGenerate { gtf, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = generateannotations(gtf).unwrap();
                println!("The command has been completed:{}", command);
            });
        }
        Commands::Countconvert { counts, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = convertcounts(counts).unwrap();
                println!("The file conversion has been done:{}", command);
            });
        }
        Commands::Differentialexpression {
            differntialexpression,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = differentialconvert(differntialexpression).unwrap();
                println!("The file has been converted:{}", command);
            });
        }
        Commands::ExonEnsembl {
            exonensembl,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = exonunwrap(exonensembl).unwrap();
                println!("The file has been written:{}", command);
            });
        }
        Commands::GeneEnsembl { ensemblid, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = geneunwrap(ensemblid).unwrap();
                println!("The file has been written:{}", command);
            });
        }
    }
}
