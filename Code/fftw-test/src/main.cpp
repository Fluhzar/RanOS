#include <fftw3.h>

#include <cmath>
#include <cstring>

#include <fstream>
#include <iostream>
#include <iomanip>
#include <limits>
#include <string>
#include <vector>

#define PI std::acos(-1)
#define N_REAL int(1024)
#define N_IMAG (N_REAL/2+1)
#define g 1.0//0.5

struct WAVHeader
{                             // (offset) = description
    uint16_t AudioFormat;     // (00) = 1
    uint16_t ChannelCount;    // (02) = 1 or 2
    uint32_t SamplingRate;    // (04) = (ex. 44.1kHz, 48kHz, 96kHz, 192kHz)
    uint32_t BytesPerSecond;  // (08) = SamplingRate * BytesPerSample
    uint16_t BytesPerSample;  // (12) = BitsPerSample/8 * ChannelCount
    uint16_t BitsPerSample;   // (14) = 8 or 16
};                            // (16) = end of structure

struct AudioData
{
    std::vector<double> data;
    double R;
};

AudioData ReadWAV(std::string const & filename)
{
    std::ifstream file(filename);

    std::vector<char> raw;

    file.seekg(0, std::ios::end);
    raw.reserve(size_t(file.tellg()));
    file.seekg(0, std::ios::beg);

    raw.assign((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());

    if(strncmp(&raw[0], "RIFF", 4) ||
       strncmp(&raw[8], "WAVE", 4) ||
       strncmp(&raw[12], "fmt ", 4) ||
       strncmp(&raw[36], "data", 4))
    {
        std::cerr << "ERROR: File not standard RIFF or WAVE format\n";
    }

    AudioData output;

    WAVHeader * header = reinterpret_cast<WAVHeader*>(&raw[20]);

    output.R = header->SamplingRate;

    char const * data = &raw[40];
    while(data < &raw[0] + raw.size())
    {
        if(header->BitsPerSample == 8)
        {
            uint8_t const * rdata = reinterpret_cast<uint8_t const *>(data);
            output.data.push_back(double(int16_t(*rdata)-128)/double(std::numeric_limits<int8_t>::max()));
        }
        else  // assume 16-bit audio is being used
        {
            int16_t const * rdata = reinterpret_cast<int16_t const *>(data);
            output.data.push_back(double(*rdata)/double(std::numeric_limits<int16_t>::max()));
        }

        data += header->BytesPerSample;
    }

    return output;
}

void fftw_test(std::string const & filename)
{
    double * in;
    fftw_complex * out;
    fftw_plan p;

    AudioData data = ReadWAV(filename);

    in = static_cast<double*>(fftw_malloc(sizeof(double) * N_REAL));
    out = static_cast<fftw_complex*>(fftw_malloc(sizeof(fftw_complex) * N_IMAG));
    p = fftw_plan_dft_r2c_1d(N_REAL, in, out, FFTW_MEASURE);

    std::ofstream outfile("out.txt");

    for(uint32_t i = 0; i < data.data.size()/N_REAL; ++i)
    {
            // Set input data
        for(uint32_t j = 0; j < N_REAL; ++j)
        {
            in[j] = data.data[i*N_REAL + j];
        }

            // Calculate FFT
        fftw_execute(p);

        double DC = std::sqrt(out[0][0]*out[0][0] + out[0][1]*out[0][1]);
        outfile << "================================================================\n";
        outfile << "DC in index " << i << ": " << DC << '\n';

        for(uint32_t j = 0; j < N_IMAG; ++j)
        {
            outfile << std::setw(4) << j+1 << ": ";
            outfile << std::setprecision(3) << std::setw(8) << std::fixed << out[j][0] << ' ';

            if(out[j][1] >= 0)
            {
                outfile << '+' << ' ';
                outfile << std::setprecision(3) << std::setw(8) << std::fixed << out[j][1] << 'j';
            }
            else
            {
                outfile << '-' << ' ';
                outfile << std::setprecision(3) << std::setw(8) << std::fixed << -out[j][1] << 'j';
            }

            if(j != 0)
                outfile << "\t\t" << "(amp,freq) = (" << std::sqrt(out[j][0]*out[j][0]+out[j][1]*out[j][1])/DC << ',' << j*double(data.R)/N_REAL << ")\n";
            else
                outfile <<'\n';
        }
    }

    fftw_destroy_plan(p);
    fftw_free(in);
    fftw_free(out);
}

int main(int argc, char * argv[])
{
    (void)(argc);
    (void)(argv);

    fftw_test(std::string(argv[1]));

    return 0;
}
